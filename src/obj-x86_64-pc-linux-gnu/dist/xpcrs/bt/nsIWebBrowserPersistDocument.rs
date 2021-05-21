//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/webbrowserpersist/nsIWebBrowserPersistDocument.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserPersistURIMap",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute unsigned long numMappedURIs; */
                    Method {
                        name: "GetNumMappedURIs",
                        params: &[Param { name: "aNumMappedURIs", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void getURIMapping (in unsigned long aIndex, out AUTF8String aMapFrom, out AUTF8String aMapTo); */
                    Method {
                        name: "GetURIMapping",
                        params: &[Param { name: "aIndex", ty: "u32" }, Param { name: "aMapFrom", ty: "*mut ::nsstring::nsACString" }, Param { name: "aMapTo", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String targetBaseURI; */
                    Method {
                        name: "GetTargetBaseURI",
                        params: &[Param { name: "aTargetBaseURI", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIWebBrowserPersistDocument",
            base: Some("nsISupports"),
            methods: Err("nostdcall is unsupported"),
        },

        Interface {
            name: "nsIWebBrowserPersistResourceVisitor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void visitResource (in nsIWebBrowserPersistDocument aDocument, in AUTF8String aURI, in nsContentPolicyType aContentPolicyType); */
                    Method {
                        name: "VisitResource",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }, Param { name: "aURI", ty: "*const ::nsstring::nsACString" }, Param { name: "aContentPolicyType", ty: "nsContentPolicyType" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void visitDocument (in nsIWebBrowserPersistDocument aDocument, in nsIWebBrowserPersistDocument aSubDocument); */
                    Method {
                        name: "VisitDocument",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }, Param { name: "aSubDocument", ty: "*const nsIWebBrowserPersistDocument" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void visitBrowsingContext (in nsIWebBrowserPersistDocument aDocument, in BrowsingContext aContext); */
                    Method {
                        name: "VisitBrowsingContext",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }, Param { name: "aContext", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void endVisit (in nsIWebBrowserPersistDocument aDocument, in nsresult aStatus); */
                    Method {
                        name: "EndVisit",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }, Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIWebBrowserPersistWriteCompletion",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onFinish (in nsIWebBrowserPersistDocument aDocument, in nsIOutputStream aStream, in ACString aContentType, in nsresult aStatus); */
                    Method {
                        name: "OnFinish",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }, Param { name: "aStream", ty: "*const nsIOutputStream" }, Param { name: "aContentType", ty: "*const ::nsstring::nsACString" }, Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIWebBrowserPersistDocumentReceiver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onDocumentReady (in nsIWebBrowserPersistDocument aDocument); */
                    Method {
                        name: "OnDocumentReady",
                        params: &[Param { name: "aDocument", ty: "*const nsIWebBrowserPersistDocument" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onError (in nsresult aFailure); */
                    Method {
                        name: "OnError",
                        params: &[Param { name: "aFailure", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

