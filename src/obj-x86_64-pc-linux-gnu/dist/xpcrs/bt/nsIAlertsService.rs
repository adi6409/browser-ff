//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/alerts/nsIAlertsService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAlertNotificationImageListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onImageReady (in nsISupports aUserData, in imgIRequest aRequest); */
                    Method {
                        name: "OnImageReady",
                        params: &[Param { name: "aUserData", ty: "*const nsISupports" }, Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onImageMissing (in nsISupports aUserData); */
                    Method {
                        name: "OnImageMissing",
                        params: &[Param { name: "aUserData", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAlertNotification",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init ([optional] in AString aName, [optional] in AString aImageURL, [optional] in AString aTitle, [optional] in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "aImageURL", ty: "*const ::nsstring::nsAString" }, Param { name: "aTitle", ty: "*const ::nsstring::nsAString" }, Param { name: "aText", ty: "*const ::nsstring::nsAString" }, Param { name: "aTextClickable", ty: "bool" }, Param { name: "aCookie", ty: "*const ::nsstring::nsAString" }, Param { name: "aDir", ty: "*const ::nsstring::nsAString" }, Param { name: "aLang", ty: "*const ::nsstring::nsAString" }, Param { name: "aData", ty: "*const ::nsstring::nsAString" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aInPrivateBrowsing", ty: "bool" }, Param { name: "aRequireInteraction", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString imageURL; */
                    Method {
                        name: "GetImageURL",
                        params: &[Param { name: "aImageURL", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString title; */
                    Method {
                        name: "GetTitle",
                        params: &[Param { name: "aTitle", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString text; */
                    Method {
                        name: "GetText",
                        params: &[Param { name: "aText", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean textClickable; */
                    Method {
                        name: "GetTextClickable",
                        params: &[Param { name: "aTextClickable", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString cookie; */
                    Method {
                        name: "GetCookie",
                        params: &[Param { name: "aCookie", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString dir; */
                    Method {
                        name: "GetDir",
                        params: &[Param { name: "aDir", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString lang; */
                    Method {
                        name: "GetLang",
                        params: &[Param { name: "aLang", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString data; */
                    Method {
                        name: "GetData",
                        params: &[Param { name: "aData", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPrincipal principal; */
                    Method {
                        name: "GetPrincipal",
                        params: &[Param { name: "aPrincipal", ty: "*mut*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI URI; */
                    Method {
                        name: "GetURI",
                        params: &[Param { name: "aURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean inPrivateBrowsing; */
                    Method {
                        name: "GetInPrivateBrowsing",
                        params: &[Param { name: "aInPrivateBrowsing", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean requireInteraction; */
                    Method {
                        name: "GetRequireInteraction",
                        params: &[Param { name: "aRequireInteraction", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean actionable; */
                    Method {
                        name: "GetActionable",
                        params: &[Param { name: "aActionable", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString source; */
                    Method {
                        name: "GetSource",
                        params: &[Param { name: "aSource", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsICancelable loadImage (in unsigned long aTimeout, in nsIAlertNotificationImageListener aListener, [optional] in nsISupports aUserData); */
                    Method {
                        name: "LoadImage",
                        params: &[Param { name: "aTimeout", ty: "u32" }, Param { name: "aListener", ty: "*const nsIAlertNotificationImageListener" }, Param { name: "aUserData", ty: "*const nsISupports" }, Param { name: "_retval", ty: "*mut*const nsICancelable" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAlertsService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void showPersistentNotification (in AString aPersistentData, in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
                    Method {
                        name: "ShowPersistentNotification",
                        params: &[Param { name: "aPersistentData", ty: "*const ::nsstring::nsAString" }, Param { name: "aAlert", ty: "*const nsIAlertNotification" }, Param { name: "aAlertListener", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void showAlert (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
                    Method {
                        name: "ShowAlert",
                        params: &[Param { name: "aAlert", ty: "*const nsIAlertNotification" }, Param { name: "aAlertListener", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void showAlertNotification (in AString aImageURL, in AString aTitle, in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in nsIObserver aAlertListener, [optional] in AString aName, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
                    Method {
                        name: "ShowAlertNotification",
                        params: &[Param { name: "aImageURL", ty: "*const ::nsstring::nsAString" }, Param { name: "aTitle", ty: "*const ::nsstring::nsAString" }, Param { name: "aText", ty: "*const ::nsstring::nsAString" }, Param { name: "aTextClickable", ty: "bool" }, Param { name: "aCookie", ty: "*const ::nsstring::nsAString" }, Param { name: "aAlertListener", ty: "*const nsIObserver" }, Param { name: "aName", ty: "*const ::nsstring::nsAString" }, Param { name: "aDir", ty: "*const ::nsstring::nsAString" }, Param { name: "aLang", ty: "*const ::nsstring::nsAString" }, Param { name: "aData", ty: "*const ::nsstring::nsAString" }, Param { name: "aPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aInPrivateBrowsing", ty: "bool" }, Param { name: "aRequireInteraction", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void closeAlert ([optional] in AString aName); */
                    Method {
                        name: "CloseAlert",
                        params: &[Param { name: "aName", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAlertsDoNotDisturb",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute bool manualDoNotDisturb; */
                    Method {
                        name: "GetManualDoNotDisturb",
                        params: &[Param { name: "aManualDoNotDisturb", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetManualDoNotDisturb",
                        params: &[Param { name: "aManualDoNotDisturb", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute bool suppressForScreenSharing; */
                    Method {
                        name: "GetSuppressForScreenSharing",
                        params: &[Param { name: "aSuppressForScreenSharing", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSuppressForScreenSharing",
                        params: &[Param { name: "aSuppressForScreenSharing", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAlertsIconData",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void showAlertWithIconData (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in uint32_t aIconSize, [array, size_is (aIconSize), const] in uint8_t aIconData); */
                    Method {
                        name: "ShowAlertWithIconData",
                        params: &[Param { name: "aAlert", ty: "*const nsIAlertNotification" }, Param { name: "aAlertListener", ty: "*const nsIObserver" }, Param { name: "aIconSize", ty: "uint32_t" }, Param { name: "aIconData", ty: "*const uint8_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIAlertsIconURI",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void showAlertWithIconURI (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in nsIURI aIconURI); */
                    Method {
                        name: "ShowAlertWithIconURI",
                        params: &[Param { name: "aAlert", ty: "*const nsIAlertNotification" }, Param { name: "aAlertListener", ty: "*const nsIObserver" }, Param { name: "aIconURI", ty: "*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

