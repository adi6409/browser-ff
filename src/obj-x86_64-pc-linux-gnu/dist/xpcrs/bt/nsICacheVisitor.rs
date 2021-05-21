//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheVisitor.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheVisitor",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean visitDevice (in string deviceID, in nsICacheDeviceInfo deviceInfo); */
                    Method {
                        name: "VisitDevice",
                        params: &[Param { name: "deviceID", ty: "*const libc::c_char" }, Param { name: "deviceInfo", ty: "*const nsICacheDeviceInfo" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean visitEntry (in string deviceID, in nsICacheEntryInfo entryInfo); */
                    Method {
                        name: "VisitEntry",
                        params: &[Param { name: "deviceID", ty: "*const libc::c_char" }, Param { name: "entryInfo", ty: "*const nsICacheEntryInfo" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICacheDeviceInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString description; */
                    Method {
                        name: "GetDescription",
                        params: &[Param { name: "aDescription", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString usageReport; */
                    Method {
                        name: "GetUsageReport",
                        params: &[Param { name: "aUsageReport", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long entryCount; */
                    Method {
                        name: "GetEntryCount",
                        params: &[Param { name: "aEntryCount", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long totalSize; */
                    Method {
                        name: "GetTotalSize",
                        params: &[Param { name: "aTotalSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long maximumSize; */
                    Method {
                        name: "GetMaximumSize",
                        params: &[Param { name: "aMaximumSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsICacheEntryInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString clientID; */
                    Method {
                        name: "GetClientID",
                        params: &[Param { name: "aClientID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString deviceID; */
                    Method {
                        name: "GetDeviceID",
                        params: &[Param { name: "aDeviceID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString key; */
                    Method {
                        name: "GetKey",
                        params: &[Param { name: "aKey", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long fetchCount; */
                    Method {
                        name: "GetFetchCount",
                        params: &[Param { name: "aFetchCount", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t lastFetched; */
                    Method {
                        name: "GetLastFetched",
                        params: &[Param { name: "aLastFetched", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t lastModified; */
                    Method {
                        name: "GetLastModified",
                        params: &[Param { name: "aLastModified", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint32_t expirationTime; */
                    Method {
                        name: "GetExpirationTime",
                        params: &[Param { name: "aExpirationTime", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long dataSize; */
                    Method {
                        name: "GetDataSize",
                        params: &[Param { name: "aDataSize", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isStreamBased (); */
                    Method {
                        name: "IsStreamBased",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

