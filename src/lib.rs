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

#[derive(Debug)]
struct StateFnIdents {
    parent_fn_ident: Option<syn::Ident>,
    enter_fn_ident: Option<syn::Ident>,
    process_fn_ident: syn::Ident,
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
        struct StateFnHdlAndParent {
            hdl: usize,
            parent_ident: Option<syn::Ident>,
        }
        let mut state_fn_hdl_and_parent = Vec::<StateFnHdlAndParent>::new();
        let mut fns = Vec::<syn::ItemFn>::new();
        let mut fn_map = HashMap::<String, usize>::new();
        while let Ok(a_fn) = input.parse::<syn::ItemFn>() {
            //println!("hsm1::parse: tol ItemFn a_fn={:#?}", a_fn);

            // Look at the attributes and check for "hsm1_state"
            for a in a_fn.attrs.iter() {
                //println!("hsm1::parse: function attributes: {:#?}", a);

                if let Some(ident) = a.path.get_ident() {
                    if ident == "hsm1_state" {
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

                        // Save the index of this function in state_fn_hdls
                        state_fn_hdl_and_parent.push(StateFnHdlAndParent {
                            hdl: fns.len(),
                            parent_ident: if let Ok(fa) = a.parse_args::<Hsm1Args>() {
                                fa.arg_ident
                            } else {
                                None
                            },
                        });
                        //println!("hsm1::parse: {} has a hsm1_state attribute, hdl={}", a_fn.sig.ident.to_string(), state_fn_hdls.last().unwrap());
                        break; // Never push more than one, although there should only be one
                    }
                }
            }

            // Add a_fn to fn_map and fns
            fn_map.insert(a_fn.sig.ident.to_string(), fns.len());
            fns.push(a_fn.clone());
        }

        let mut state_fn_idents_map = HashMap::<String, usize>::new();
        let mut state_fn_idents = Vec::<StateFnIdents>::new();
        for state_fn_info in state_fn_hdl_and_parent {
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
/// ```ignore // Ignore because clippy warnings of neeless main
/// use proc_macro_hsm1::{handled, hsm1, hsm1_state, not_handled};
///
/// // These two use's needed as hsm1 is dependent upon them.
/// // How can hsm1 proc_macro signify the dependency?
/// use std::collections::VecDeque;
/// use state_result::*;
///
/// hsm1!(
///     struct MyFsm {
///         initial_counter: u64,
///     }
///
///     #[hsm1_state]
///     fn initial(&mut self) -> StateResult {
///         // Mutate the state
///         self.initial_counter += 1;
///
///         // Let the parent state handle all invocations
///         handled!()
///     }
/// );
///
/// hsm1!(
///     struct MyHsm {
///         base_counter: u64,
///         initial_counter: u64,
///     }
///
///     #[hsm1_state]
///     fn base(&mut self) -> StateResult {
///         // Mutate the state
///         self.base_counter += 1;
///
///         // Return the desired StateResult
///         handled!()
///     }
///
///     #[hsm1_state(base)]
///     fn initial(&mut self) -> StateResult {
///         // Mutate the state
///         self.initial_counter += 1;
///
///         // Let the parent state handle all invocations
///         not_handled!()
///     }
/// );
///
/// fn main() {
///     let mut fsm = MyFsm::new();
///
///     fsm.dispatch();
///     println!( "fsm: fsm intial_counter={}", fsm.initial_counter);
///     assert_eq!(fsm.initial_counter, 1);
///
///     let mut hsm = MyHsm::new();
///
///     hsm.dispatch();
///     println!(
///         "hsm: hsm base_counter={} intial_counter={}",
///         hsm.base_counter, hsm.initial_counter
///     );
///     assert_eq!(hsm.base_counter, 1);
///     assert_eq!(hsm.initial_counter, 1);
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
                    parent, hsm_ident, process_fn_ident
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

    let output = quote!(
        // We need these but can't import them multiple times
        // How do we automatically derive proc_macro dependencies?
        //use std::collections::VecDeque;
        //use sm::{StateResult, StateFnsHdl};

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
                let mut enter_hdl = self.smi.current_state_fns_hdl;
                loop {
                    //println!("initial_enter_fns_hdls: push(enter_hdl={})", enter_hdl);
                    self.smi.enter_fns_hdls.push(enter_hdl);
                    enter_hdl = if let Some(hdl) = self.smi.state_fns[enter_hdl].parent {
                        hdl
                    } else {
                        break;
                    };
                }
            }

            // Starting at self.current_state_fns_hdl generate the
            // list of StateInfo that we're going to exit. If exit_sentinel is None
            // then exit from current_state_fns_hdl and all of its parents.
            // If exit_sentinel is Some then exit from the current state_fns_hdl
            // up to but not including the exit_sentinel.
            fn setup_exit_fns_hdls(&mut self, exit_sentinel: Option<usize>) {

                let mut exit_hdl = self.smi.current_state_fns_hdl;
                loop {
                    //println!("setup_exit_fns_hdls: push_back(exit_hdl={})", exit_hdl);
                    self.smi.exit_fns_hdls.push_back(exit_hdl);

                    if Some(exit_hdl) == exit_sentinel {
                        // This handles the special case where we're transitioning to ourself
                        //println!("setup_exit_fns_hdls: reached sentinel, done");
                        return;
                    }

                    // Getting parents handle
                    exit_hdl = if let Some(hdl) = self.smi.state_fns[exit_hdl].parent {
                        hdl
                    } else {
                        // No parent we're done
                        //println!("setup_exit_fns_hdls: No more parents, done");
                        return;
                    };

                    if Some(exit_hdl) == exit_sentinel {
                        // Reached the exit sentinel so we're done
                        return;
                    }
                }
            }

            // Setup exit_fns_hdls and enter_fns_hdls.
            fn setup_exit_enter_fns_hdls(&mut self, next_state_hdl: usize) {
                let mut cur_hdl = next_state_hdl;

                // Setup the enter vector
                let exit_sentinel = loop {
                    //println!("setup_exit_enter_fns_hdls: push(cur_hdl={})", cur_hdl);
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

                // Setup the exit vector
                self.setup_exit_fns_hdls(exit_sentinel);
            }

            // TODO: Not sure this is worth it, if it is consider adding hsm_name()
            fn state_name(&self) -> &str {
                &self.smi.state_fns[self.smi.current_state_fns_hdl].name
            }

            fn dispatch_hdl(&mut self, hdl: StateFnsHdl) {
                if self.smi.current_state_changed {
                    // Execute the enter functions
                    while let Some(enter_hdl) = self.smi.enter_fns_hdls.pop() {
                        if let Some(state_enter) = self.smi.state_fns[enter_hdl].enter {
                            //println!("enter while: enter_hdl={} call state_enter={}", enter_hdl, state_enter as usize);
                            (state_enter)(self);
                            self.smi.state_fns[enter_hdl].active = true;
                            //println!("enter while: retf state_enter={}", state_enter as usize);
                        } else {
                            //println!("enter while: enter_hdl={} NO ENTER FN", enter_hdl);
                        }
                    }

                    self.smi.current_state_changed = false;
                }

                let mut transition_dest_hdl = None;

                match (self.smi.state_fns[hdl].process)(self) {
                    StateResult::NotHandled => {
                        // This handles the special case where we're transitioning to ourself
                        if let Some(parent_hdl) = self.smi.state_fns[hdl].parent {
                            self.dispatch_hdl(parent_hdl);
                        } else {
                            // TODO: Consider calling a "default_handler" when NotHandled and no parent
                        }
                    }
                    StateResult::Handled => {
                        // Nothing to do
                    }
                    StateResult::TransitionTo(next_state_hdl) => {
                        self.setup_exit_enter_fns_hdls(next_state_hdl);
                        self.smi.current_state_changed = true;
                        transition_dest_hdl = Some(next_state_hdl);
                    }
                }

                if self.smi.current_state_changed {
                    while let Some(exit_hdl) = self.smi.exit_fns_hdls.pop_front() {
                        if let Some(state_exit) = self.smi.state_fns[exit_hdl].exit {
                            (state_exit)(self);
                        }
                    }
                }

                if let Some(hdl) = transition_dest_hdl {
                    // Change the previous and current state_fns_hdl after we've
                    // preformed the exit routines so state_name is correct.
                    self.smi.previous_state_fns_hdl = self.smi.current_state_fns_hdl;
                    self.smi.current_state_fns_hdl = hdl;
                }
            }

            pub fn dispatch(&mut self) {
                self.dispatch_hdl(self.smi.current_state_fns_hdl);
            }
        }

        type #state_fn = fn(&mut #hsm_ident, /* &Protocol1 */) -> StateResult;
        type #state_fn_enter = fn(&mut #hsm_ident, /* &Protocol1 */);
        type #state_fn_exit = fn(&mut #hsm_ident, /* &Protocol1 */);

        //#[derive(Debug)]
        struct #state_info {
            name: String, // TODO: Remove or add StateMachineInfo::name?
            parent: Option<StateFnsHdl>,
            enter: Option<#state_fn_enter>,
            process: #state_fn,
            exit: Option<#state_fn_exit>,
            active: bool,
        }

        //#[derive(Debug)]
        struct #state_machine_info {
            //name: String, // TODO: add StateMachineInfo::name
            state_fns: [#state_info; #hsm_state_fns_len],
            enter_fns_hdls: Vec<StateFnsHdl>,
            exit_fns_hdls: VecDeque<StateFnsHdl>,
            current_state_fns_hdl: StateFnsHdl,
            previous_state_fns_hdl: StateFnsHdl,
            current_state_changed: bool,
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
                    enter_fns_hdls: Vec::<StateFnsHdl>::with_capacity(#hsm_state_fns_len),
                    exit_fns_hdls: VecDeque::<StateFnsHdl>::with_capacity(#hsm_state_fns_len),
                    current_state_fns_hdl: #initial_state_hdl,
                    previous_state_fns_hdl: #initial_state_hdl,
                    current_state_changed: true,
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

    quote!(StateResult::TransitionTo(#item_ts2)).into()
}

#[proc_macro]
pub fn handled(_item: TokenStream) -> TokenStream {
    //println!("proc_macro handled!: item={:?}", item);
    quote!(StateResult::Handled).into()
}

#[proc_macro]
pub fn not_handled(_item: TokenStream) -> TokenStream {
    //println!("proc_macro not_handled!: item={:?}", item);
    quote!(StateResult::NotHandled).into()
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
        if let Some(ident_segment) = node.path.segments.last() {
            // The last segment is the name of the macro
            if ident_segment.ident == "transition_to" {
                // Found our macro, transition_to

                // Get the first token; aka: parameter to the function
                let mut iter = node.tokens.clone().into_iter();
                if let Some(token) = iter.next() {
                    if iter.next().is_some() {
                        // TODO: improve error handling
                        panic!("transition_to! may have only one parameter, the name of the state")
                    }
                    let parameter = token.to_string();
                    if let Some(hdl) = self.hsm_state_fn_ident_map.get(&parameter) {
                        //println!("Visitor::visit_macro_mut: Found {} in {} with index {}", parameter, self.hsm_ident, hdl);
                        node.tokens = quote!(#hdl);
                        return;
                    } else {
                        panic!("No state named {} in {}", parameter, self.hsm_ident);
                    }
                } else {
                    // TODO: improve error handling
                    panic!("transition_to! must have one parameter, the name of the state")
                }
            }
        }

        // Delegate to the default impl to visit any nested macros.
        visit_mut::visit_macro_mut(self, node);

        //println!("Visitor::visit_macro_mut:- hsm_ident={} node={:?}",hsm_ident, node);
    }
}
