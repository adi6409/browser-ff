//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/txmgr/nsITransactionManager.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsITransactionManager",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [can_run_script] void doTransaction (in nsITransaction aTransaction); */
                    Method {
                        name: "DoTransaction",
                        params: &[Param { name: "aTransaction", ty: "*const nsITransaction" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void undoTransaction (); */
                    Method {
                        name: "UndoTransaction",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void redoTransaction (); */
                    Method {
                        name: "RedoTransaction",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void clear (); */
                    Method {
                        name: "Clear",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearUndoStack (); */
                    Method {
                        name: "ClearUndoStack",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void clearRedoStack (); */
                    Method {
                        name: "ClearRedoStack",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* [can_run_script] void beginBatch (in nsISupports aData); */
                    Method {
                        name: "BeginBatch",
                        params: &[Param { name: "aData", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void endBatch (in boolean aAllowEmpty); */
                    Method {
                        name: "EndBatch",
                        params: &[Param { name: "aAllowEmpty", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long numberOfUndoItems; */
                    Method {
                        name: "GetNumberOfUndoItems",
                        params: &[Param { name: "aNumberOfUndoItems", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long numberOfRedoItems; */
                    Method {
                        name: "GetNumberOfRedoItems",
                        params: &[Param { name: "aNumberOfRedoItems", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute long maxTransactionCount; */
                    Method {
                        name: "GetMaxTransactionCount",
                        params: &[Param { name: "aMaxTransactionCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetMaxTransactionCount",
                        params: &[Param { name: "aMaxTransactionCount", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void batchTopUndo (); */
                    Method {
                        name: "BatchTopUndo",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeTopUndo (); */
                    Method {
                        name: "RemoveTopUndo",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* nsITransaction peekUndoStack (); */
                    Method {
                        name: "PeekUndoStack",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITransaction" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsITransaction peekRedoStack (); */
                    Method {
                        name: "PeekRedoStack",
                        params: &[Param { name: "_retval", ty: "*mut *const nsITransaction" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void AddListener (in nsITransactionListener aListener); */
                    Method {
                        name: "AddListener",
                        params: &[Param { name: "aListener", ty: "*const nsITransactionListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void RemoveListener (in nsITransactionListener aListener); */
                    Method {
                        name: "RemoveListener",
                        params: &[Param { name: "aListener", ty: "*const nsITransactionListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

