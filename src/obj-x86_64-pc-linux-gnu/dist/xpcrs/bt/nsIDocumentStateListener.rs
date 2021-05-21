//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIDocumentStateListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocumentStateListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] void NotifyDocumentWillBeDestroyed (); */
                    Method {
                        name: "NotifyDocumentWillBeDestroyed",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void NotifyDocumentStateChanged (in boolean aNowDirty); */
                    Method {
                        name: "NotifyDocumentStateChanged",
                        params: &[Param { name: "aNowDirty", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

