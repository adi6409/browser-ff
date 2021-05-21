//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIWebProgressListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIWebProgressListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onStateChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aStateFlags, in nsresult aStatus); */
                    Method {
                        name: "OnStateChange",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aStateFlags", ty: "u32" }, Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onProgressChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in long aCurSelfProgress, in long aMaxSelfProgress, in long aCurTotalProgress, in long aMaxTotalProgress); */
                    Method {
                        name: "OnProgressChange",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aCurSelfProgress", ty: "i32" }, Param { name: "aMaxSelfProgress", ty: "i32" }, Param { name: "aCurTotalProgress", ty: "i32" }, Param { name: "aMaxTotalProgress", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onLocationChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in nsIURI aLocation, [optional] in unsigned long aFlags); */
                    Method {
                        name: "OnLocationChange",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aLocation", ty: "*const nsIURI" }, Param { name: "aFlags", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onStatusChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in nsresult aStatus, in wstring aMessage); */
                    Method {
                        name: "OnStatusChange",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aStatus", ty: "::nserror::nsresult" }, Param { name: "aMessage", ty: "*const i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onSecurityChange (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aState); */
                    Method {
                        name: "OnSecurityChange",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aState", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onContentBlockingEvent (in nsIWebProgress aWebProgress, in nsIRequest aRequest, in unsigned long aEvent); */
                    Method {
                        name: "OnContentBlockingEvent",
                        params: &[Param { name: "aWebProgress", ty: "*const nsIWebProgress" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aEvent", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

