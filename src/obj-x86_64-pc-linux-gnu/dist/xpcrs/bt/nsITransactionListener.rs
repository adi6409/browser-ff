//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/txmgr/nsITransactionListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITransactionListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean willDo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
                    Method {
                        name: "WillDo",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void didDo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aDoResult); */
                    Method {
                        name: "DidDo",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "aDoResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean willUndo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
                    Method {
                        name: "WillUndo",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void didUndo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aUndoResult); */
                    Method {
                        name: "DidUndo",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "aUndoResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean willRedo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
                    Method {
                        name: "WillRedo",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void didRedo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aRedoResult); */
                    Method {
                        name: "DidRedo",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTransaction", ty: "*const nsITransaction" }, Param { name: "aRedoResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean willBeginBatch (in nsITransactionManager aManager); */
                    Method {
                        name: "WillBeginBatch",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void didBeginBatch (in nsITransactionManager aManager, in nsresult aResult); */
                    Method {
                        name: "DidBeginBatch",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean willEndBatch (in nsITransactionManager aManager); */
                    Method {
                        name: "WillEndBatch",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void didEndBatch (in nsITransactionManager aManager, in nsresult aResult); */
                    Method {
                        name: "DidEndBatch",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean willMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge); */
                    Method {
                        name: "WillMerge",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTopTransaction", ty: "*const nsITransaction" }, Param { name: "aTransactionToMerge", ty: "*const nsITransaction" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void didMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge, in boolean aDidMerge, in nsresult aMergeResult); */
                    Method {
                        name: "DidMerge",
                        params: &[Param { name: "aManager", ty: "*const nsITransactionManager" }, Param { name: "aTopTransaction", ty: "*const nsITransaction" }, Param { name: "aTransactionToMerge", ty: "*const nsITransaction" }, Param { name: "aDidMerge", ty: "bool" }, Param { name: "aMergeResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

