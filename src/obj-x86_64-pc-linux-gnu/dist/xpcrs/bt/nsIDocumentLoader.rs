//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIDocumentLoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocumentLoader",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void stop (); */
                    Method {
                        name: "Stop",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISupports container; */
                    Method {
                        name: "GetContainer",
                        params: &[Param { name: "aContainer", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsILoadGroup loadGroup; */
                    Method {
                        name: "GetLoadGroup",
                        params: &[Param { name: "aLoadGroup", ty: "*mut*const nsILoadGroup" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIChannel documentChannel; */
                    Method {
                        name: "GetDocumentChannel",
                        params: &[Param { name: "aDocumentChannel", ty: "*mut*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

