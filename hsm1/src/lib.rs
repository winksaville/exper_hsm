#![feature(core_intrinsics)]
///! Hierarchical State Machine proc_macro
use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::visit_mut::{self, VisitMut};
use syn::{parse_macro_input, Macro, Result};

#[proc_macro_attribute]
pub fn hsm1_state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //println!("proc_macro_attribute hsm1_state: attr={:#?}", attr);
    //println!("proc_macro_attribute hsm1_state: item={:#?}", item);
    item
}

#[derive(Debug)]
struct Hsm1 {
    hsm_ident: syn::Ident,
    hsm_fields: Vec<syn::Field>,
    hsm_fns: Vec<syn::ItemFn>,
    #[allow(unused)]
    hsm_state_fn_ident_map: HashMap<String, usize>,
    hsm_state_fn_idents: Vec<StateFnIdents>,
}

#[derive(Debug, Clone)]
enum MsgType {
    MtTypePath { tp: syn::TypePath },
    MtTypeReference { tr: syn::TypeReference },
}

#[derive(Debug)]
struct StateFnIdents {
    parent_fn_ident: Option<syn::Ident>,
    enter_fn_ident: Option<syn::Ident>,
    process_fn_ident: syn::Ident,
    process_fn_msg_type: MsgType,
    exit_fn_ident: Option<syn::Ident>,
}

impl Parse for Hsm1 {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("hsm1::parse:+");
        //println!("hsm1::parse: input={:#?}", input);

        let item_struct = input.parse::<syn::ItemStruct>()?;
        //println!("hsm1::parse: item_struct={:#?}", item_struct);

        // Parse all of the hsm1 data fields
        let fields: Vec<syn::Field> = match item_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named.iter().cloned().collect(),
            _ => {
                let err = syn::Error::new_spanned(item_struct, "hsm1::parse: expecting hsm struct");
                return Err(err);
            }
        };
        //println!("hsm1::parse: fields={:#?}", fields);

        // The only thing that should remain are functions
        #[derive(Debug)]
        struct StateFnInfo {
            hdl: usize,
            parent_ident: Option<syn::Ident>,
            #[allow(unused)]
            msg_type: MsgType,
        }
        let mut state_fn_infos = Vec::<StateFnInfo>::new();
        let mut fns = Vec::<syn::ItemFn>::new();
        let mut fn_map = HashMap::<String, usize>::new();

        // TODO: Gracefully handle when input.parse::<syn::ItemFn> returns an Err!
        while let Ok(a_fn) = input.parse::<syn::ItemFn>() {
            //println!("hsm1::parse: tol ItemFn a_fn={:#?}", a_fn);

            // Look at the attributes and check for "hsm1_state"
            for a in a_fn.attrs.iter() {
                //println!("hsm1::parse: function attributes: {:#?}", a);

                if let Some(ident) = a.path.get_ident() {
                    if ident == "hsm1_state" {
                        // There zero or one parameter to the hsm1_state and
                        // we're not interested in other atributes, so break;
                        // TODO: There is probably a better way to implement
                        // optional parameters to proc_macro_attribute. The problem
                        // is if there is no arguments "a.parse_args" returns err, but
                        // in hsm1Args::parse I also handle the notion of no args in
                        // that it also returns None thus this feels over complicated.
                        #[derive(Debug)]
                        struct Hsm1Args {
                            #[allow(unused)]
                            arg_ident: Option<syn::Ident>,
                        }

                        impl Parse for Hsm1Args {
                            fn parse(input: ParseStream) -> Result<Self> {
                                // There should only be one ident
                                let name = if let Ok(id) = input.parse() {
                                    Some(id)
                                } else {
                                    None
                                };
                                Ok(Hsm1Args { arg_ident: name })
                            }
                        }

                        // Enable to print the sig.input as pairs
                        //for a_pair in a_fn.sig.inputs.pairs() {
                        //    match a_pair {
                        //        syn::punctuated::Pair::Punctuated(fn_arg, _) => {
                        //            println!("Pair::Puncated: {fn_arg:#?}");
                        //        }
                        //        syn::punctuated::Pair::End(fn_arg) => {
                        //            println!("Pair::End: {fn_arg:#?}");
                        //        }
                        //    }
                        //}

                        // Now parse the arguments there should be two arguments
                        //   &mut self,  msg: &mut MsgType

                        let len_input_pairs = a_fn.sig.inputs.pairs().len();
                        //println!("hsm1::parse: fn {} inputs.pairs.len={}", a_fn.sig.ident, len_input_pairs);
                        if len_input_pairs != 2 {
                            // TODO: Improve error handling
                            panic!("All hsm1_state functions must have two parameters, `fn xxx(&mut self, msg: MsgType)`");
                        }

                        // Iternate over the "inputs" which are the parameters
                        let mut sig_iter = a_fn.sig.inputs.pairs(); // why is sig_iter need to be mut?

                        // Verify first argument is "&mut self"
                        if let Some(pair) = sig_iter.next() {
                            match pair {
                                syn::punctuated::Pair::Punctuated(self_arg, _) => {
                                    //println!("self_arg={self_arg:#?}");
                                    match self_arg {
                                        syn::FnArg::Receiver(rcvr) => {
                                            if !rcvr.attrs.is_empty()
                                                || rcvr.reference.is_none()
                                                || rcvr.mutability.is_none()
                                            {
                                                panic!(
                                                    "Expected first parameter to be `&mut self`"
                                                );
                                            }
                                        }
                                        syn::FnArg::Typed(_) => {
                                            panic!("Expected first parameter to be `&mut self`");
                                        }
                                    }
                                }
                                syn::punctuated::Pair::End(_) => {
                                    panic!("Expected &mut self as first parameter to an state funtion (SHOULD NOT HAPPEN as len_input_pairs == 2)");
                                }
                            }
                        } else {
                            panic!("No parameters, expected two parameters; &mut self, msg &MsgType (SHOULD NOT HAPPEN, as len_input_pairs == 2)");
                        }

                        // Get msg Type in the signature
                        let msg_type = if let Some(pair) = sig_iter.next() {
                            match pair {
                                syn::punctuated::Pair::Punctuated(_, _) => {
                                    panic!("Too many parameters, expected two parameters; &mut self, msg &MsgType (SHOULD NOT HAPPEN, as len_input_pairs == 2)");
                                }
                                syn::punctuated::Pair::End(last_arg) => {
                                    //println!("last_arg={last_arg:#?}");
                                    match last_arg {
                                        syn::FnArg::Typed(pt) => {
                                            match &*pt.ty {
                                                syn::Type::Reference(tr) => {
                                                    //println!("tr={tr:#?}");
                                                    MsgType::MtTypeReference { tr: tr.clone() }
                                                }
                                                syn::Type::Path(tp) => {
                                                    //println!("tp={tp:#?}");
                                                    MsgType::MtTypePath { tp: tp.clone() }
                                                }
                                                //syn::Type::Array(_) => todo!(),
                                                //syn::Type::BareFn(_) => todo!(),
                                                //syn::Type::Group(_) => todo!(),
                                                //syn::Type::ImplTrait(_) => todo!(),
                                                //syn::Type::Infer(_) => todo!(),
                                                //syn::Type::Macro(_) => todo!(),
                                                //syn::Type::Never(_) => todo!(),
                                                //syn::Type::Paren(_) => todo!(),
                                                //syn::Type::Ptr(_) => todo!(),
                                                //syn::Type::Slice(_) => todo!(),
                                                //syn::Type::TraitObject(_) => todo!(),
                                                //syn::Type::Tuple(_) => todo!(),
                                                //syn::Type::Verbatim(_) => todo!(),
                                                _ => {
                                                    panic!("Expected msg type");
                                                }
                                            }
                                        }
                                        syn::FnArg::Receiver(_) => {
                                            // TODO Improve error handling
                                            panic!("Expected `msg: MsgType` as last parameter, a `self` is not allowed");
                                        }
                                    }
                                }
                            }
                        } else {
                            panic!("Expected &mut self as first parameter of an state funtion");
                        };

                        // Save the index of this function in state_fn_hdls
                        state_fn_infos.push(StateFnInfo {
                            hdl: fns.len(),
                            parent_ident: if let Ok(fa) = a.parse_args::<Hsm1Args>() {
                                fa.arg_ident
                            } else {
                                None
                            },
                            msg_type,
                        });
                        //println!("hsm1::parse: state_fn_info {:#?}", state_fn_infos.last());

                        break;
                    }
                }
            }

            // Add a_fn to fn_map and fns
            fn_map.insert(a_fn.sig.ident.to_string(), fns.len());
            fns.push(a_fn.clone());
        }

        let mut state_fn_idents_map = HashMap::<String, usize>::new();
        let mut state_fn_idents = Vec::<StateFnIdents>::new();
        for state_fn_info in state_fn_infos {
            let item_fn = &fns[state_fn_info.hdl];
            let process_fn_ident = item_fn.sig.ident.clone();

            let enter_fn_ident = new_ident(process_fn_ident.clone(), "_enter");
            let exit_fn_ident = new_ident(process_fn_ident.clone(), "_exit");

            let enter_fn_ident_opt = if fn_map.get(enter_fn_ident.to_string().as_str()).is_some() {
                Some(enter_fn_ident)
            } else {
                None
            };

            let exit_fn_ident_opt = if fn_map.get(exit_fn_ident.to_string().as_str()).is_some() {
                Some(exit_fn_ident)
            } else {
                None
            };

            state_fn_idents_map.insert(process_fn_ident.to_string(), state_fn_idents.len());
            state_fn_idents.push(StateFnIdents {
                parent_fn_ident: state_fn_info.parent_ident,
                enter_fn_ident: enter_fn_ident_opt,
                process_fn_ident,
                process_fn_msg_type: state_fn_info.msg_type,
                exit_fn_ident: exit_fn_ident_opt,
            });
        }

        //println!("hsm1::parse:-");
        Ok(Hsm1 {
            hsm_ident: item_struct.ident.clone(),
            hsm_fields: fields,
            hsm_fns: fns,
            hsm_state_fn_ident_map: state_fn_idents_map,
            hsm_state_fn_idents: state_fn_idents,
        })
    }
}

/// hsm1 proc_macro
///
/// # Examples
///
/// Two examples; MyFsm is the simplest FSM with just one state.
/// MyHsm is the simplest HSM with two states, initial with base
/// as its parent.
///
/// ```ignore // Used to supress clippy warnings, there's got to be a better way :(
/// use proc_macro_hsm1::{handled, hsm1, hsm1_state, not_handled};
///
/// // These two use's needed as hsm1 is dependent upon them.
/// // How can hsm1 proc_macro signify the dependency?
/// use state_result::*;
///
/// pub enum Messages {
///     Add {
///         a_field: u64,
///     },
///     Get {
///         sum_a_field: u64,
///     },
/// }
///
/// hsm1!(
///     struct MyFsm {
///         initial_counter: u64,
///     }
///
///     #[hsm1_state]
///     fn initial(&mut self, _msg: &mut Messages) -> StateResult!() {
///         // Mutate the state
///         self.initial_counter += 1;
///
///         handled!()
///     }
/// );
///
/// hsm1!(
///     struct MyHsm {
///         sum_a_field: u64,
///         base_counter: u64,
///         initial_counter: u64,
///     }
///
///     #[hsm1_state]
///     fn base(&mut self, msg: &mut Messages) -> StateResult!() {
///         // Mutate the state
///         self.base_counter += 1;
///         match msg {
///             Messages::Get { sum_a_field } => {
///                 *sum_a_field = self.sum_a_field;
///                 handled!()
///             }
///             _ => not_handled!()
///         }
///     }
///
///     #[hsm1_state(base)]
///     fn initial(&mut self, msg: &mut Messages) -> StateResult!() {
///         // Mutate the state
///         self.initial_counter += 1;
///
///         match msg {
///             Messages::Add { a_field } => {
///                 self.sum_a_field += *a_field;
///                 handled!()
///             }
///             _ => not_handled!()
///         }
///     }
/// );
///
/// fn main() {
///     let mut fsm = MyFsm::new();
///
///     let mut msg = Messages::Add {
///         a_field: 1,
///     };
///     fsm.dispatch(&mut msg);
///     println!( "fsm: intial_counter={}", fsm.initial_counter);
///     assert_eq!(fsm.initial_counter, 1);
///
///     let mut hsm = MyHsm::new();
///
///     let mut msg = Messages::Add {
///         a_field: 10,
///     };
///     hsm.dispatch(&mut msg);
///     println!(
///         "hsm: sum_a_field={} base_counter={} intial_counter={}",
///         hsm.sum_a_field, hsm.base_counter, hsm.initial_counter
///     );
///     assert_eq!(hsm.sum_a_field, 10);
///     assert_eq!(hsm.base_counter, 0);
///     assert_eq!(hsm.initial_counter, 1);
///
///     let mut msg = Messages::Get {
///         sum_a_field: 0,
///     };
///     hsm.dispatch(&mut msg);
///     println!(
///         "hsm: sum_a_field={} base_counter={} intial_counter={}",
///         hsm.sum_a_field, hsm.base_counter, hsm.initial_counter
///     );
///     match msg {
///         Messages::Get{ sum_a_field } => {
///             assert_eq!(sum_a_field, hsm.sum_a_field );
///             assert_eq!(sum_a_field, 10);
///         }
///         _ => {
///             panic!("Should Not Happen");
///         }
///     }
///     assert_eq!(hsm.sum_a_field, 10);
///     assert_eq!(hsm.base_counter, 1);
///     assert_eq!(hsm.initial_counter, 2);
/// }
/// ```
#[proc_macro]
pub fn hsm1(input: TokenStream) -> TokenStream {
    //println!("hsm1:+");

    //println!("hsm1:+ input={:#?}", &input);
    let in_ts = input;

    let hsm = parse_macro_input!(in_ts as Hsm1);
    //println!("hsm1: hsm={:#?}", hsm);

    let hsm_ident = hsm.hsm_ident;
    //println!("hsm1: hsm_ident={:#?}", hsm_ident);

    let hsm_fields = hsm.hsm_fields;
    //println!("hsm1: hsm_fields={:#?}", hsm_fields);

    let hsm_fns = hsm.hsm_fns;
    //println!("hsm1: hsm_fns={:#?}", hsm_fns);

    let hsm_state_fn_ident_map = hsm.hsm_state_fn_ident_map;
    //println!("hsm1: hsm_state_fn_ident_map={:?}", _hsm_state_fn_ident_map);

    let state_fn = new_ident(hsm_ident.clone(), "StateFn");
    let state_fn_enter = new_ident(hsm_ident.clone(), "StateFnEnter");
    let state_fn_exit = new_ident(hsm_ident.clone(), "StateFnExit");
    let state_info = new_ident(hsm_ident.clone(), "StateInfo");
    let state_machine_info = new_ident(hsm_ident.clone(), "StateMachineInfo");
    let mut state_fn_msg_type_opt: Option<MsgType> = None;

    let hsm_state_fn_idents = hsm.hsm_state_fn_idents;
    let mut hsm_state_fns = Vec::<syn::ExprStruct>::new();
    let mut hsm_initial_state_fns_hdl: Option<usize> = None;

    for sfn in &hsm_state_fn_idents {
        //println!("hsm1: sf={:#?}", sfn);

        let process_fn_ident = sfn.process_fn_ident.clone();
        //println!("hsm1: process_fn_ident={}", process_fn_ident);
        if process_fn_ident == "initial" {
            assert_eq!(hsm_initial_state_fns_hdl, None);
            hsm_initial_state_fns_hdl = Some(hsm_state_fns.len());
            state_fn_msg_type_opt = Some(sfn.process_fn_msg_type.clone());
        }

        let opt_fn_ident = |ident: Option<syn::Ident>| match ident {
            Some(ident) => quote!(Some(#hsm_ident::#ident)),
            None => quote!(None),
        };
        let parent_hdl: TokenStream2 = if let Some(parent_ident) = &sfn.parent_fn_ident {
            let parent = parent_ident.to_string();
            if let Some(hdl) = hsm_state_fn_ident_map.get(&parent) {
                quote!(Some(#hdl))
            } else {
                // TODO: Improve error handling
                panic!(
                    "{}::{} is not defined and cannot be parent of {}",
                    hsm_ident, parent, process_fn_ident
                );
            }
        } else {
            quote!(None)
        };
        //println!("hsm1: parent_fn={}", parent_fn);
        let enter_fn = opt_fn_ident(sfn.enter_fn_ident.clone());
        //println!("hsm1: enter_fn={}", enter_fn);
        let exit_fn = opt_fn_ident(sfn.exit_fn_ident.clone());
        //println!("hsm1: exit_fn={}", exit_fn);

        let ts: TokenStream2 = quote!(
            #state_info {
                name: stringify!(#process_fn_ident).to_owned(),
                parent: #parent_hdl,
                enter: #enter_fn,
                process: #hsm_ident::#process_fn_ident,
                exit: #exit_fn,
                active: false,
            }
        );
        let sf_es = syn::parse2::<syn::ExprStruct>(ts);
        if let Ok(es) = sf_es {
            hsm_state_fns.push(es);
        }
    }
    //println!("hsm1: hsm_state_fns:\n{:#?}", hsm_state_fns);

    let hsm_state_fns_len = hsm_state_fns.len();
    let initial_state_hdl = if let Some(hdl) = hsm_initial_state_fns_hdl {
        hdl
    } else {
        // TODO: Better error handling
        panic!("No initial state");
    };
    //println!("hsm1: hsm_state_fns_len: {} initial_state_hdl={}", hsm_state_fns_len, initial_state_hdl);

    let mut visitor = Visitor {
        hsm_ident: hsm_ident.clone(),
        hsm_state_fn_ident_map,
    };

    let mut converted_fns = Vec::<syn::ItemFn>::new();
    for a_fn in hsm_fns.iter() {
        //println!("hsm1: visiting a_fn={:?}", a_fn.sig.ident);
        let mut mut_a_fn = a_fn.clone();
        visitor.visit_item_fn_mut(&mut mut_a_fn);
        converted_fns.push(mut_a_fn);
    }
    //println!("hsm1: converted_fns={:#?}", converted_fns);

    let state_fn_msg_type: TokenStream2 = if let Some(msg_type) = state_fn_msg_type_opt {
        //println!("msg_type={msg_type:?}");
        match msg_type {
            MsgType::MtTypePath { tp } => quote!(#tp),
            MsgType::MtTypeReference { tr } => quote!(#tr),
        }
    } else {
        panic!("No msg type");
    };
    //println!("state_fn_msg_type_path={state_fn_msg_type_path:?}");
    //println!("hsm_ident={hsm_ident:?}");

    let output = quote!(

        // error: implementation of `Debug` is not general enough
        //   --> proc-macro-hsm1/src/main.rs:8:1
        //    |
        // 8  | / hsm1!(
        // 9  | |     #[derive(Debug)]
        // 10 | |     struct MyFsm {
        // 11 | |         initial_counter: u64,
        // ...  |
        // 21 | |     }
        // 22 | | );
        //    | |_^ implementation of `Debug` is not general enough
        //    |
        //    = note: `Debug` would have to be implemented for the type `for<'r> fn(&'r mut MyFsm)`
        //    = note: ...but `Debug` is actually implemented for the type `fn(&'0 mut MyFsm)`, for some specific lifetime `'0`
        //    = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)
        //#[derive(Debug)]
        #[derive(Default)]
        struct #hsm_ident {
            smi: #state_machine_info,

            #(
                #[allow(unused)]
                #hsm_fields
            ),*
        }

        impl #hsm_ident {
            pub fn new() -> Self {
                let mut smi: #hsm_ident = Default::default();

                smi.initial_enter_fns_hdls();

                smi
            }

            #(
                #[allow(unused)]
                #converted_fns
            )*

            // When the state machine starts there will be no fn's to
            // exit so we initialize only the enter_fns_hdls.
            fn initial_enter_fns_hdls(&mut self) {
                println!("initial_enter_fns_hdls:+ enter_fns_hdls len={}", self.smi.enter_fns_hdls.len());
                let mut enter_hdl = self.smi.current_state_fns_hdl;
                loop {
                    println!("initial_enter_fns_hdls: push(enter_hdl={})", enter_hdl);
                    self.smi.enter_fns_hdls.push(enter_hdl);
                    enter_hdl = if let Some(hdl) = self.smi.state_fns[enter_hdl].parent {
                        hdl
                    } else {
                        break;
                    };
                }
                println!("initial_enter_fns_hdls:- enter_fns_hdls len={}", self.smi.enter_fns_hdls.len());
            }

            // Starting at self.current_state_fns_hdl generate the
            // list of StateInfo that we're going to exit. If exit_sentinel is None
            // then exit from current_state_fns_hdl and all of its parents.
            // If exit_sentinel is Some then exit from the current state_fns_hdl
            // up to but not including the exit_sentinel.
            fn setup_exit_fns_hdls(&mut self, exit_sentinel: Option<usize>) {

                let mut exit_hdl = self.smi.current_state_fns_hdl;
                println!("setup_exit_fns_hdls:  Starting EXIT  pushing len={}", self.smi.exit_fns_hdls.len());
                loop {
                    println!("setup_exit_fns_hdls: EXIT push_back(exit_hdl={})", exit_hdl);
                    self.smi.exit_fns_hdls.push_back(exit_hdl);

                    if Some(exit_hdl) == exit_sentinel {
                        // This handles the special case where we're transitioning to ourself
                        //println!("setup_exit_fns_hdls: reached sentinel, done");
                        break;
                    }

                    // Getting parents handle
                    exit_hdl = if let Some(hdl) = self.smi.state_fns[exit_hdl].parent {
                        hdl
                    } else {
                        // No parent we're done
                        //println!("setup_exit_fns_hdls: No more parents, done");
                        break;
                    };

                    if Some(exit_hdl) == exit_sentinel {
                        // Reached the exit sentinel so we're done
                        break;
                    }
                }
                println!("setup_exit_fns_hdls: Completed EXIT  pushing len={}", self.smi.exit_fns_hdls.len());
            }

            // Setup exit_fns_hdls and enter_fns_hdls.
            fn setup_exit_enter_fns_hdls(&mut self, next_state_hdl: usize) {
                let mut cur_hdl = next_state_hdl;

                // Setup the enter vector
                println!("setup_exit_enter_fns_hdls:  Starting ENTER pushing len={}", self.smi.enter_fns_hdls.len());
                let exit_sentinel = loop {
                    println!("setup_exit_enter_fns_hdls: ENTER push(cur_hdl={})", cur_hdl);
                    self.smi.enter_fns_hdls.push(cur_hdl);

                    cur_hdl = if let Some(hdl) = self.smi.state_fns[cur_hdl].parent {
                        //println!("setup_exit_enter_fns_hdls: cur_hdl={}", cur_hdl);
                        hdl
                    } else {
                        // Exit state_fns[self.current_state_fns_hdl] and all its parents
                        //println!("setup_exit_enter_fns_hdls: No more parents");
                        break None;
                    };

                    if self.smi.state_fns[cur_hdl].active {
                        // Exit state_fns[self.current_state_fns_hdl] and
                        // parents upto but excluding state_fns[cur_hdl]
                        //println!("setup_exit_enter_fns_hdls: set exit_sentinel={}", cur_hdl);
                        break Some(cur_hdl);
                    }
                };
                println!("setup_exit_enter_fns_hdls: Completed ENTER pushing len={}", self.smi.enter_fns_hdls.len());

                // Setup the exit vector
                self.setup_exit_fns_hdls(exit_sentinel);
            }

            // TODO: Not sure this is worth it, if it is consider adding hsm_name()
            fn state_name(&self) -> &str {
                &self.smi.state_fns[self.smi.current_state_fns_hdl].name
            }

            fn set_transition_dest_hdl(&mut self, hdl: usize) {
                println!("set_transition_dest_hdl: hdl={}", hdl);
                self.smi.transition_dest_hdl = Some(hdl);
            }

            fn dispatch_hdl(&mut self, msg: #state_fn_msg_type, hdl: usize) {
                println!("dispatch_hdl {}:+", hdl);
                if self.smi.current_state_changed && !self.smi.enter_fns_hdls.is_empty() {
                    println!("dispatch_hdl {}: Starting  enter loop len={}", hdl, self.smi.enter_fns_hdls.len());
                    // Execute the enter functions
                    while let Some(enter_hdl) = self.smi.enter_fns_hdls.pop() {
                        println!("dispatch_hdl {}: Top Of Loop      for enter_hdl={} len={}", hdl, enter_hdl, self.smi.enter_fns_hdls.len());
                        if let Some(state_enter) = self.smi.state_fns[enter_hdl].enter {
                            println!("dispatch_hdl {}: call state_enter for enter_hdl={}", hdl, enter_hdl);
                            (state_enter)(self, msg);
                            self.smi.state_fns[enter_hdl].active = true;
                            println!("dispatch_hdl {}: retf state_enter for enter_hdl={}", hdl, enter_hdl);
                        } else {
                            println!("dispatch_hdl {}: NO state_enter   for enter_hdl={}", hdl, enter_hdl);
                        }
                    }

                    self.smi.current_state_changed = false;
                    self.smi.transition_dest_hdl = None;
                    println!("dispatch_hdl {}: Completed enter loop len={}", hdl, self.smi.enter_fns_hdls.len());
                }

                println!("dispatch_hdl {}: call process", hdl);
                match (self.smi.state_fns[hdl].process)(self, msg) {
                    state_result::StateResult::NotHandled => {
                        // This handles the special case where we're transitioning to ourself
                        if let Some(parent_hdl) = self.smi.state_fns[hdl].parent {
                            println!("dispatch_hdl {}: retf process, NotHandled, call dispatch_hdl({})", hdl, parent_hdl);
                            self.dispatch_hdl(msg, parent_hdl);
                            println!("dispatch_hdl {}: retf process, NotHandled, retf dispatch_hdl({})", hdl, parent_hdl);
                        } else {
                            // TODO: Consider calling a "default_handler" when NotHandled and no parent
                            println!("dispatch_hdl {}: retf process, NotHandled no parent", hdl);
                        }
                    }
                    state_result::StateResult::Handled => {
                        // Nothing to do
                        println!("dispatch_hdl {}: retf process, Handled", hdl);
                    }
                    state_result::StateResult::TransitionTo(dest_hdl) => {
                        println!("dispatch_hdl {}: retf process, TransitionTo({})", hdl, dest_hdl);
                        self.set_transition_dest_hdl(dest_hdl);
                    }
                }

                if let Some(dest_hdl) = self.smi.transition_dest_hdl {
                    self.setup_exit_enter_fns_hdls(dest_hdl);
                    self.smi.current_state_changed = true;
                }

                if self.smi.current_state_changed && !self.smi.exit_fns_hdls.is_empty() {
                    println!("dispatch_hdl {}: Starting  exit loop len={}", hdl, self.smi.exit_fns_hdls.len());
                    while let Some(exit_hdl) = self.smi.exit_fns_hdls.pop_front() {
                        println!("dispatch_hdl {}: Top Of Loop     for exit_hdl={}", hdl, exit_hdl);
                        if let Some(state_exit) = self.smi.state_fns[exit_hdl].exit {
                            println!("dispatch_hdl {}: call state_exit for exit_hdl={}", hdl, exit_hdl);
                            (state_exit)(self, msg);
                            self.smi.state_fns[exit_hdl].active = true;
                            println!("dispatch_hdl {}: retf state_exit for exit_hdl={}", hdl, exit_hdl);
                        } else {
                            println!("dispatch_hdl {}: NO state_exit   for exit_hdl={}", hdl, exit_hdl);
                        }
                    }
                    println!("dispatch_hdl {}: Completed exit loop len={}", hdl, self.smi.exit_fns_hdls.len());
                }

                if let Some(dest_hdl) = self.smi.transition_dest_hdl {
                    // Change the previous and current state_fns_hdl after we've
                    // preformed the exit routines so state_name is correct.
                    self.smi.previous_state_fns_hdl = self.smi.current_state_fns_hdl;
                    self.smi.current_state_fns_hdl = dest_hdl;
                    //println!("dispatch_hdl {}: transitioned, updated previous {} and current {} state hdls", hdl, self.smi.previous_state_fns_hdl, self.smi.current_state_fns_hdl);
                }

                println!("dispatch_hdl {}:-", hdl);
            }

            pub fn dispatch(&mut self, msg: #state_fn_msg_type) {
                println!("dispatch {}:+", self.smi.current_state_fns_hdl);
                self.dispatch_hdl(msg, self.smi.current_state_fns_hdl);
                println!("dispatch {}:-", self.smi.current_state_fns_hdl);
            }
        }

        type #state_fn = fn(&mut #hsm_ident, #state_fn_msg_type) -> state_result::StateResult;
        type #state_fn_enter = fn(&mut #hsm_ident, #state_fn_msg_type);
        type #state_fn_exit = fn(&mut #hsm_ident, #state_fn_msg_type);

        //#[derive(Debug)]
        struct #state_info {
            name: String, // TODO: Remove or add StateMachineInfo::name?
            parent: Option<state_result::StateFnsHdl>,
            enter: Option<#state_fn_enter>,
            process: #state_fn,
            exit: Option<#state_fn_exit>,
            active: bool,
        }

        //#[derive(Debug)]
        struct #state_machine_info {
            //name: String, // TODO: add StateMachineInfo::name
            state_fns: [#state_info; #hsm_state_fns_len],
            enter_fns_hdls: Vec<state_result::StateFnsHdl>,
            exit_fns_hdls: std::collections::VecDeque<state_result::StateFnsHdl>,
            current_state_fns_hdl: state_result::StateFnsHdl,
            previous_state_fns_hdl: state_result::StateFnsHdl,
            current_state_changed: bool,
            transition_dest_hdl: Option<state_result::StateFnsHdl>,
        }

        impl Default for #state_machine_info {
            fn default() -> Self {
                Self::new()
            }
        }

        impl #state_machine_info {
            fn new() -> Self {
                Self {
                    state_fns: [
                        #(
                            #hsm_state_fns
                        ),*
                    ],
                    enter_fns_hdls: Vec::<state_result::StateFnsHdl>::with_capacity(#hsm_state_fns_len),
                    exit_fns_hdls: std::collections::VecDeque::<state_result::StateFnsHdl>::with_capacity(#hsm_state_fns_len),
                    current_state_fns_hdl: #initial_state_hdl,
                    previous_state_fns_hdl: #initial_state_hdl,
                    current_state_changed: true,
                    transition_dest_hdl: None,
                }
            }
        }
    );
    //println!("hsm1: output={:#?}", output);

    //println!("hsm1:-");
    output.into()
}

#[proc_macro]
pub fn transition_to(item: TokenStream) -> TokenStream {
    let item_ts2: TokenStream2 = item.into();
    //println!("proc_macro transition_to!: item_ts2={:?}", item_ts2);

    quote!(state_result::StateResult::TransitionTo(#item_ts2)).into()
}

#[proc_macro]
pub fn set_transition_dest(item: TokenStream) -> TokenStream {
    let item_ts2: TokenStream2 = item.into();
    //println!("proc_macro set_transition_dest!: item_ts2={:?}", item_ts2);

    quote!(self.set_transition_dest_hdl(#item_ts2)).into()
}

#[proc_macro]
pub fn handled(_item: TokenStream) -> TokenStream {
    //println!("proc_macro handled!: item={:?}", item);
    quote!(state_result::StateResult::Handled).into()
}

#[proc_macro]
pub fn not_handled(_item: TokenStream) -> TokenStream {
    //println!("proc_macro not_handled!: item={:?}", item);
    quote!(state_result::StateResult::NotHandled).into()
}

#[allow(non_snake_case)]
#[proc_macro]
pub fn StateResult(_item: TokenStream) -> TokenStream {
    //println!("proc_macro not_handled!: item={:?}", item);
    quote!(state_result::StateResult).into()
}

fn new_ident(ident: syn::Ident, suffix: &str) -> syn::Ident {
    syn::Ident::new(
        (ident.to_string() + suffix.to_owned().as_str()).as_str(),
        ident.span(),
    )
}

struct Visitor {
    hsm_ident: syn::Ident,
    hsm_state_fn_ident_map: HashMap<String, usize>,
}

impl VisitMut for Visitor {
    // Invoke visit_item_fn_mut which will invoke vist_macro_mut for
    // each macro in the funtion. The code here will convert each
    // transtion_to!(state_fn_name) to transition_to!(state_fn_index).
    fn visit_macro_mut(&mut self, node: &mut Macro) {
        fn ident_to_hdl(visitor: &mut Visitor, node: &mut Macro) {
            // Found our macro, transition_to

            // Get the first token; aka: parameter to the function
            let mut iter = node.tokens.clone().into_iter();
            if let Some(token) = iter.next() {
                if iter.next().is_some() {
                    // TODO: improve error handling
                    panic!("transition_to! may have only one parameter, the name of the state")
                }
                let parameter = token.to_string();
                if let Some(hdl) = visitor.hsm_state_fn_ident_map.get(&parameter) {
                    //println!("Visitor::visit_macro_mut: Found {} in {} with index {}", parameter, self.hsm_ident, hdl);
                    node.tokens = quote!(#hdl);
                    return;
                } else {
                    // TODO: improve error handling
                    panic!("No state named {} in {}", parameter, visitor.hsm_ident);
                }
            } else {
                // TODO: improve error handling
                panic!("transition_to! must have one parameter, the name of the state")
            }
        }

        if let Some(ident_segment) = node.path.segments.last() {
            // The last segment is the name of the macro
            match ident_segment.ident.to_string().as_str() {
                "transition_to" => return ident_to_hdl(self, node),
                "set_transition_dest" => return ident_to_hdl(self, node),
                _ => {}
            }
        }

        // Delegate to the default impl to visit any nested macros.
        visit_mut::visit_macro_mut(self, node);

        //println!("Visitor::visit_macro_mut:- hsm_ident={} node={:?}",hsm_ident, node);
    }
}
