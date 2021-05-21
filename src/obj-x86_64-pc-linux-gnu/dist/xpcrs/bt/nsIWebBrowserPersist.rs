//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/webbrowserpersist/nsIWebBrowserPersist.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserPersist",
            base: Some("nsICancelable"),
            methods: Ok(&[
                    /* attribute unsigned long persistFlags; */
                    Method {
                        name: "GetPersistFlags",
                        params: &[Param { name: "aPersistFlags", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPersistFlags",
                        params: &[Param { name: "aPersistFlags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long currentState; */
                    Method {
                        name: "GetCurrentState",
                        params: &[Param { name: "aCurrentState", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsresult result; */
                    Method {
                        name: "GetResult",
                        params: &[Param { name: "aResult", ty: "*mut ::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIWebProgressListener progressListener; */
                    Method {
                        name: "GetProgressListener",
                        params: &[Param { name: "aProgressListener", ty: "*mut*const nsIWebProgressListener" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetProgressListener",
                        params: &[Param { name: "aProgressListener", ty: "*const nsIWebProgressListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void saveURI (in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in unsigned long aCacheKey, in nsIReferrerInfo aReferrerInfo, in nsICookieJarSettings aCookieJarSettings, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsContentPolicyType aContentPolicyType, in nsILoadContext aPrivacyContext); */
                    Method {
                        name: "SaveURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCacheKey", ty: "u32" }, Param { name: "aReferrerInfo", ty: "*const nsIReferrerInfo" }, Param { name: "aCookieJarSettings", ty: "*const nsICookieJarSettings" }, Param { name: "aPostData", ty: "*const nsIInputStream" }, Param { name: "aExtraHeaders", ty: "*const libc::c_char" }, Param { name: "aFile", ty: "*const nsISupports" }, Param { name: "aContentPolicyType", ty: "nsContentPolicyType" }, Param { name: "aPrivacyContext", ty: "*const nsILoadContext" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void savePrivacyAwareURI (in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in unsigned long aCacheKey, in nsIReferrerInfo aReferrerInfo, in nsICookieJarSettings aCookieJarSettings, in nsIInputStream aPostData, in string aExtraHeaders, in nsISupports aFile, in nsContentPolicyType aContentPolicyType, in boolean aIsPrivate); */
                    Method {
                        name: "SavePrivacyAwareURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCacheKey", ty: "u32" }, Param { name: "aReferrerInfo", ty: "*const nsIReferrerInfo" }, Param { name: "aCookieJarSettings", ty: "*const nsICookieJarSettings" }, Param { name: "aPostData", ty: "*const nsIInputStream" }, Param { name: "aExtraHeaders", ty: "*const libc::c_char" }, Param { name: "aFile", ty: "*const nsISupports" }, Param { name: "aContentPolicyType", ty: "nsContentPolicyType" }, Param { name: "aIsPrivate", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void saveChannel (in nsIChannel aChannel, in nsISupports aFile); */
                    Method {
                        name: "SaveChannel",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aFile", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void saveDocument (in nsISupports aDocument, in nsISupports aFile, in nsISupports aDataPath, in string aOutputContentType, in unsigned long aEncodingFlags, in unsigned long aWrapColumn); */
                    Method {
                        name: "SaveDocument",
                        params: &[Param { name: "aDocument", ty: "*const nsISupports" }, Param { name: "aFile", ty: "*const nsISupports" }, Param { name: "aDataPath", ty: "*const nsISupports" }, Param { name: "aOutputContentType", ty: "*const libc::c_char" }, Param { name: "aEncodingFlags", ty: "u32" }, Param { name: "aWrapColumn", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancelSave (); */
                    Method {
                        name: "CancelSave",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

