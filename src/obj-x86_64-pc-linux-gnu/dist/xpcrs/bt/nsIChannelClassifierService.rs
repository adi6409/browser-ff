//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIChannelClassifierService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierBlockedChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint8_t reason; */
                    Method {
                        name: "GetReason",
                        params: &[Param { name: "aReason", ty: "*mut uint8_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString tables; */
                    Method {
                        name: "GetTables",
                        params: &[Param { name: "aTables", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString url; */
                    Method {
                        name: "GetUrl",
                        params: &[Param { name: "aUrl", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint64_t tabId; */
                    Method {
                        name: "GetTabId",
                        params: &[Param { name: "aTabId", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute uint64_t channelId; */
                    Method {
                        name: "GetChannelId",
                        params: &[Param { name: "aChannelId", ty: "*mut uint64_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean isPrivateBrowsing; */
                    Method {
                        name: "GetIsPrivateBrowsing",
                        params: &[Param { name: "aIsPrivateBrowsing", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString topLevelUrl; */
                    Method {
                        name: "GetTopLevelUrl",
                        params: &[Param { name: "aTopLevelUrl", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unblock (); */
                    Method {
                        name: "Unblock",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void allow (); */
                    Method {
                        name: "Allow",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIChannelClassifierService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void addListener (in nsIObserver aObserver); */
                    Method {
                        name: "AddListener",
                        params: &[Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeListener (in nsIObserver aObserver); */
                    Method {
                        name: "RemoveListener",
                        params: &[Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

