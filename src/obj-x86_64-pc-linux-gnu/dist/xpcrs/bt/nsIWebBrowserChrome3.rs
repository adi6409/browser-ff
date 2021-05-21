//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowserChrome3.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebBrowserChrome3",
            base: Some("nsIWebBrowserChrome"),
            methods: Ok(&[
                    /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in Node linkNode, in boolean isAppTab); */
                    Method {
                        name: "OnBeforeLinkTraversal",
                        params: &[Param { name: "originalTarget", ty: "*const ::nsstring::nsAString" }, Param { name: "linkURI", ty: "*const nsIURI" }, Param { name: "linkNode", ty: "*const libc::c_void" }, Param { name: "isAppTab", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool shouldLoadURI (in nsIDocShell aDocShell, in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in boolean aHasPostData, in nsIPrincipal aTriggeringPrincipal, in nsIContentSecurityPolicy aCsp); */
                    Method {
                        name: "ShouldLoadURI",
                        params: &[Param { name: "aDocShell", ty: "*const nsIDocShell" }, Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aReferrerInfo", ty: "*const nsIReferrerInfo" }, Param { name: "aHasPostData", ty: "bool" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCsp", ty: "*const nsIContentSecurityPolicy" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* bool shouldLoadURIInThisProcess (in nsIURI aURI); */
                    Method {
                        name: "ShouldLoadURIInThisProcess",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

