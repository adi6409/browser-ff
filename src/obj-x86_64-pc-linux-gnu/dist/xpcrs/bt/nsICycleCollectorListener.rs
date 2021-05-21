//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsICycleCollectorListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICycleCollectorHandler",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void noteRefCountedObject (in ACString aAddress, in unsigned long aRefCount, in ACString aObjectDescription); */
                    Method {
                        name: "NoteRefCountedObject",
                        params: &[Param { name: "aAddress", ty: "*const ::nsstring::nsACString" }, Param { name: "aRefCount", ty: "u32" }, Param { name: "aObjectDescription", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void noteGCedObject (in ACString aAddress, in boolean aMarked, in ACString aObjectDescription, in ACString aCompartmentAddress); */
                    Method {
                        name: "NoteGCedObject",
                        params: &[Param { name: "aAddress", ty: "*const ::nsstring::nsACString" }, Param { name: "aMarked", ty: "bool" }, Param { name: "aObjectDescription", ty: "*const ::nsstring::nsACString" }, Param { name: "aCompartmentAddress", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void noteEdge (in ACString aFromAddress, in ACString aToAddress, in ACString aEdgeName); */
                    Method {
                        name: "NoteEdge",
                        params: &[Param { name: "aFromAddress", ty: "*const ::nsstring::nsACString" }, Param { name: "aToAddress", ty: "*const ::nsstring::nsACString" }, Param { name: "aEdgeName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void describeRoot (in ACString aAddress, in unsigned long aKnownEdges); */
                    Method {
                        name: "DescribeRoot",
                        params: &[Param { name: "aAddress", ty: "*const ::nsstring::nsACString" }, Param { name: "aKnownEdges", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void describeGarbage (in ACString aAddress); */
                    Method {
                        name: "DescribeGarbage",
                        params: &[Param { name: "aAddress", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICycleCollectorLogSink",
            base: Some("nsISupports"),
            methods: Err("native type FILE unsupported"),
        },

        Interface {
            name: "nsICycleCollectorListener",
            base: Some("nsISupports"),
            methods: Err("native type nsCycleCollectorLogger unsupported"),
        },

        ]; D}

