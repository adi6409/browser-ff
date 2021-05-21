//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIXULBrowserWindow.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIXULBrowserWindow",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setOverLink (in AString link); */
                    Method {
                        name: "SetOverLink",
                        params: &[Param { name: "link", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in Node linkNode, in boolean isAppTab); */
                    Method {
                        name: "OnBeforeLinkTraversal",
                        params: &[Param { name: "originalTarget", ty: "*const ::nsstring::nsAString" }, Param { name: "linkURI", ty: "*const nsIURI" }, Param { name: "linkNode", ty: "*const libc::c_void" }, Param { name: "isAppTab", ty: "bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void showTooltip (in long x, in long y, in AString tooltip, in AString direction, in Element browser); */
                    Method {
                        name: "ShowTooltip",
                        params: &[Param { name: "x", ty: "i32" }, Param { name: "y", ty: "i32" }, Param { name: "tooltip", ty: "*const ::nsstring::nsAString" }, Param { name: "direction", ty: "*const ::nsstring::nsAString" }, Param { name: "browser", ty: "*const libc::c_void" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void hideTooltip (); */
                    Method {
                        name: "HideTooltip",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* uint32_t getTabCount (); */
                    Method {
                        name: "GetTabCount",
                        params: &[Param { name: "_retval", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

