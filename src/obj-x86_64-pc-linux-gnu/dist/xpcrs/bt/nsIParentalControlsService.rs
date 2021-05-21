//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/parentalcontrols/nsIParentalControlsService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIParentalControlsService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean parentalControlsEnabled; */
                    Method {
                        name: "GetParentalControlsEnabled",
                        params: &[Param { name: "aParentalControlsEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean blockFileDownloadsEnabled; */
                    Method {
                        name: "GetBlockFileDownloadsEnabled",
                        params: &[Param { name: "aBlockFileDownloadsEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isAllowed (in short aAction, [optional] in nsIURI aUri); */
                    Method {
                        name: "IsAllowed",
                        params: &[Param { name: "aAction", ty: "i16" }, Param { name: "aUri", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean loggingEnabled; */
                    Method {
                        name: "GetLoggingEnabled",
                        params: &[Param { name: "aLoggingEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void log (in short aEntryType, in boolean aFlag, in nsIURI aSource, [optional] in nsIFile aTarget); */
                    Method {
                        name: "Log",
                        params: &[Param { name: "aEntryType", ty: "i16" }, Param { name: "aFlag", ty: "bool" }, Param { name: "aSource", ty: "*const nsIURI" }, Param { name: "aTarget", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

