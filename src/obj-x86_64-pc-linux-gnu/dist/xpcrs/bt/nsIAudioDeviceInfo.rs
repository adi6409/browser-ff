//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/nsIAudioDeviceInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAudioDeviceInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString name; */
                    Method {
                        name: "GetName",
                        params: &[Param { name: "aName", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString groupId; */
                    Method {
                        name: "GetGroupId",
                        params: &[Param { name: "aGroupId", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString vendor; */
                    Method {
                        name: "GetVendor",
                        params: &[Param { name: "aVendor", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short type; */
                    Method {
                        name: "GetType",
                        params: &[Param { name: "aType", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short state; */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "aState", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short preferred; */
                    Method {
                        name: "GetPreferred",
                        params: &[Param { name: "aPreferred", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short supportedFormat; */
                    Method {
                        name: "GetSupportedFormat",
                        params: &[Param { name: "aSupportedFormat", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned short defaultFormat; */
                    Method {
                        name: "GetDefaultFormat",
                        params: &[Param { name: "aDefaultFormat", ty: "*mut u16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long maxChannels; */
                    Method {
                        name: "GetMaxChannels",
                        params: &[Param { name: "aMaxChannels", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long defaultRate; */
                    Method {
                        name: "GetDefaultRate",
                        params: &[Param { name: "aDefaultRate", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long maxRate; */
                    Method {
                        name: "GetMaxRate",
                        params: &[Param { name: "aMaxRate", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long minRate; */
                    Method {
                        name: "GetMinRate",
                        params: &[Param { name: "aMinRate", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long maxLatency; */
                    Method {
                        name: "GetMaxLatency",
                        params: &[Param { name: "aMaxLatency", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long minLatency; */
                    Method {
                        name: "GetMinLatency",
                        params: &[Param { name: "aMinLatency", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

