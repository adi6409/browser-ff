//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/shell/nsIShellService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIShellService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean isDefaultBrowser ([optional] in boolean aForAllTypes); */
                    Method {
                        name: "IsDefaultBrowser",
                        params: &[Param { name: "aForAllTypes", ty: "bool" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setDefaultBrowser (in boolean aClaimAllTypes, in boolean aForAllUsers); */
                    Method {
                        name: "SetDefaultBrowser",
                        params: &[Param { name: "aClaimAllTypes", ty: "bool" }, Param { name: "aForAllUsers", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setDesktopBackground (in Element aElement, in long aPosition, in ACString aImageName); */
                    Method {
                        name: "SetDesktopBackground",
                        params: &[Param { name: "aElement", ty: "*const libc::c_void" }, Param { name: "aPosition", ty: "i32" }, Param { name: "aImageName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute unsigned long desktopBackgroundColor; */
                    Method {
                        name: "GetDesktopBackgroundColor",
                        params: &[Param { name: "aDesktopBackgroundColor", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDesktopBackgroundColor",
                        params: &[Param { name: "aDesktopBackgroundColor", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

