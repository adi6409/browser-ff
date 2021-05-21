//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webvtt/nsIWebVTTParserWrapper.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebVTTParserWrapper",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void loadParser (in mozIDOMWindow window); */
                    Method {
                        name: "LoadParser",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindow" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void parse (in ACString data); */
                    Method {
                        name: "Parse",
                        params: &[Param { name: "data", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void flush (); */
                    Method {
                        name: "Flush",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void watch (in nsIWebVTTListener callback); */
                    Method {
                        name: "Watch",
                        params: &[Param { name: "callback", ty: "*const nsIWebVTTListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancel (); */
                    Method {
                        name: "Cancel",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* DocumentFragment convertCueToDOMTree (in mozIDOMWindow window, in nsISupports cue); */
                    Method {
                        name: "ConvertCueToDOMTree",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindow" }, Param { name: "cue", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void processCues (in mozIDOMWindow window, in nsIVariant cues, in nsISupports overlay, in nsISupports controls); */
                    Method {
                        name: "ProcessCues",
                        params: &[Param { name: "window", ty: "*const mozIDOMWindow" }, Param { name: "cues", ty: "*const nsIVariant" }, Param { name: "overlay", ty: "*const nsISupports" }, Param { name: "controls", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

