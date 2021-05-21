//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIWebProgress.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebProgress",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addProgressListener (in nsIWebProgressListener aListener, in unsigned long aNotifyMask); */
                    Method {
                        name: "AddProgressListener",
                        params: &[Param { name: "aListener", ty: "*const nsIWebProgressListener" }, Param { name: "aNotifyMask", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeProgressListener (in nsIWebProgressListener aListener); */
                    Method {
                        name: "RemoveProgressListener",
                        params: &[Param { name: "aListener", ty: "*const nsIWebProgressListener" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute mozIDOMWindowProxy DOMWindow; */
                    Method {
                        name: "GetDOMWindow",
                        params: &[Param { name: "aDOMWindow", ty: "*mut*const mozIDOMWindowProxy" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isTopLevel; */
                    Method {
                        name: "GetIsTopLevel",
                        params: &[Param { name: "aIsTopLevel", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isLoadingDocument; */
                    Method {
                        name: "GetIsLoadingDocument",
                        params: &[Param { name: "aIsLoadingDocument", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long loadType; */
                    Method {
                        name: "GetLoadType",
                        params: &[Param { name: "aLoadType", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIEventTarget target; */
                    Method {
                        name: "GetTarget",
                        params: &[Param { name: "aTarget", ty: "*mut*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetTarget",
                        params: &[Param { name: "aTarget", ty: "*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

