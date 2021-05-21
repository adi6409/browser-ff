//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISelectionListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISelectionListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] void notifySelectionChanged (in Document doc, in Selection sel, in short reason); */
                    Method {
                        name: "NotifySelectionChanged",
                        params: &[Param { name: "doc", ty: "*const libc::c_void" }, Param { name: "sel", ty: "*const libc::c_void" }, Param { name: "reason", ty: "i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

