//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/html/nsIFormSubmitObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIFormSubmitObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void notifyInvalidSubmit (in HTMLFormElement formNode, in Array<Element> invalidElements); */
                    Method {
                        name: "NotifyInvalidSubmit",
                        params: &[Param { name: "formNode", ty: "*const libc::c_void" }, Param { name: "invalidElements", ty: "*const thin_vec::ThinVec<*const libc::c_void>" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

