//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIBrowserDOMWindow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIOpenURIInFrameParams",
            base: Some("nsISupports"),
            methods: Err("specialtype jsval unsupported"),
        },

        Interface {
            name: "nsIBrowserDOMWindow",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* BrowsingContext createContentWindow (in nsIURI aURI, in nsIOpenWindowInfo aOpenWindowInfo, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal, [optional] in nsIContentSecurityPolicy aCsp); */
                    Method {
                        name: "CreateContentWindow",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aOpenWindowInfo", ty: "*const nsIOpenWindowInfo" }, Param { name: "aWhere", ty: "i16" }, Param { name: "aFlags", ty: "i32" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCsp", ty: "*const nsIContentSecurityPolicy" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element createContentWindowInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in AString aName); */
                    Method {
                        name: "CreateContentWindowInFrame",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "params", ty: "*const nsIOpenURIInFrameParams" }, Param { name: "aWhere", ty: "i16" }, Param { name: "aFlags", ty: "i32" }, Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* BrowsingContext openURI (in nsIURI aURI, in nsIOpenWindowInfo aOpenWindowInfo, in short aWhere, in long aFlags, in nsIPrincipal aTriggeringPrincipal, [optional] in nsIContentSecurityPolicy aCsp); */
                    Method {
                        name: "OpenURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aOpenWindowInfo", ty: "*const nsIOpenWindowInfo" }, Param { name: "aWhere", ty: "i16" }, Param { name: "aFlags", ty: "i32" }, Param { name: "aTriggeringPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aCsp", ty: "*const nsIContentSecurityPolicy" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* Element openURIInFrame (in nsIURI aURI, in nsIOpenURIInFrameParams params, in short aWhere, in long aFlags, in AString aName); */
                    Method {
                        name: "OpenURIInFrame",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "params", ty: "*const nsIOpenURIInFrameParams" }, Param { name: "aWhere", ty: "i16" }, Param { name: "aFlags", ty: "i32" }, Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut *const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean canClose (); */
                    Method {
                        name: "CanClose",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long tabCount; */
                    Method {
                        name: "GetTabCount",
                        params: &[Param { name: "aTabCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

