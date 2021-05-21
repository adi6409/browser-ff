//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/shistory/nsISHistoryListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISHistoryListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void OnHistoryNewEntry (in nsIURI aNewURI, in long aOldIndex); */
                    Method {
                        name: "OnHistoryNewEntry",
                        params: &[Param { name: "aNewURI", ty: "*const nsIURI" }, Param { name: "aOldIndex", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean OnHistoryReload (); */
                    Method {
                        name: "OnHistoryReload",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void OnHistoryGotoIndex (); */
                    Method {
                        name: "OnHistoryGotoIndex",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void OnHistoryPurge (); */
                    Method {
                        name: "OnHistoryPurge",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void OnHistoryReplaceEntry (); */
                    Method {
                        name: "OnHistoryReplaceEntry",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void OnContentViewerEvicted (in unsigned long aNumEvicted); */
                    Method {
                        name: "OnContentViewerEvicted",
                        params: &[Param { name: "aNumEvicted", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

