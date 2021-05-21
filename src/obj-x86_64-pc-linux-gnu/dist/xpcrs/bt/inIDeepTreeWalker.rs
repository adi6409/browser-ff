//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/inspector/inIDeepTreeWalker.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "inIDeepTreeWalker",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute boolean showAnonymousContent; */
                    Method {
                        name: "GetShowAnonymousContent",
                        params: &[Param { name: "aShowAnonymousContent", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetShowAnonymousContent",
                        params: &[Param { name: "aShowAnonymousContent", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean showSubDocuments; */
                    Method {
                        name: "GetShowSubDocuments",
                        params: &[Param { name: "aShowSubDocuments", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetShowSubDocuments",
                        params: &[Param { name: "aShowSubDocuments", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean showDocumentsAsNodes; */
                    Method {
                        name: "GetShowDocumentsAsNodes",
                        params: &[Param { name: "aShowDocumentsAsNodes", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetShowDocumentsAsNodes",
                        params: &[Param { name: "aShowDocumentsAsNodes", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in Node aRoot, in unsigned long aWhatToShow); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aRoot", ty: "*const libc::c_void" }, Param { name: "aWhatToShow", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute Node root; */
                    Method {
                        name: "GetRoot",
                        params: &[Param { name: "aRoot", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long whatToShow; */
                    Method {
                        name: "GetWhatToShow",
                        params: &[Param { name: "aWhatToShow", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute Node currentNode; */
                    Method {
                        name: "GetCurrentNode",
                        params: &[Param { name: "aCurrentNode", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetCurrentNode",
                        params: &[Param { name: "aCurrentNode", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Node parentNode (); */
                    Method {
                        name: "ParentNode",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Node firstChild (); */
                    Method {
                        name: "FirstChild",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Node lastChild (); */
                    Method {
                        name: "LastChild",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Node previousSibling (); */
                    Method {
                        name: "PreviousSibling",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Node nextSibling (); */
                    Method {
                        name: "NextSibling",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Node previousNode (); */
                    Method {
                        name: "PreviousNode",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Node nextNode (); */
                    Method {
                        name: "NextNode",
                        params: &[Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

