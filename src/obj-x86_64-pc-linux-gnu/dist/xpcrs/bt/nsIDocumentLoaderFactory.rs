//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIDocumentLoaderFactory.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDocumentLoaderFactory",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIContentViewer createInstance (in string aCommand, in nsIChannel aChannel, in nsILoadGroup aLoadGroup, in ACString aContentType, in nsIDocShell aContainer, in nsISupports aExtraInfo, out nsIStreamListener aDocListenerResult); */
                    Method {
                        name: "CreateInstance",
                        params: &[Param { name: "aCommand", ty: "*const libc::c_char" }, Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aLoadGroup", ty: "*const nsILoadGroup" }, Param { name: "aContentType", ty: "*const ::nsstring::nsACString" }, Param { name: "aContainer", ty: "*const nsIDocShell" }, Param { name: "aExtraInfo", ty: "*const nsISupports" }, Param { name: "aDocListenerResult", ty: "*mut*const nsIStreamListener" }, Param { name: "_retval", ty: "*mut*const nsIContentViewer" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIContentViewer createInstanceForDocument (in nsISupports aContainer, in Document aDocument, in string aCommand); */
                    Method {
                        name: "CreateInstanceForDocument",
                        params: &[Param { name: "aContainer", ty: "*const nsISupports" }, Param { name: "aDocument", ty: "*const libc::c_void" }, Param { name: "aCommand", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut*const nsIContentViewer" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

