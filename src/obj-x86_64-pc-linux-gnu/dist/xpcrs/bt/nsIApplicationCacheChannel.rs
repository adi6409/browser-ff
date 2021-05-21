//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCacheChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIApplicationCacheChannel",
            base: Some("nsIApplicationCacheContainer"),
            methods: Ok(&[
                    /* readonly attribute boolean loadedFromApplicationCache; */
                    Method {
                        name: "GetLoadedFromApplicationCache",
                        params: &[Param { name: "aLoadedFromApplicationCache", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean inheritApplicationCache; */
                    Method {
                        name: "GetInheritApplicationCache",
                        params: &[Param { name: "aInheritApplicationCache", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetInheritApplicationCache",
                        params: &[Param { name: "aInheritApplicationCache", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean chooseApplicationCache; */
                    Method {
                        name: "GetChooseApplicationCache",
                        params: &[Param { name: "aChooseApplicationCache", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetChooseApplicationCache",
                        params: &[Param { name: "aChooseApplicationCache", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void markOfflineCacheEntryAsForeign (); */
                    Method {
                        name: "MarkOfflineCacheEntryAsForeign",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIApplicationCache applicationCacheForWrite; */
                    Method {
                        name: "GetApplicationCacheForWrite",
                        params: &[Param { name: "aApplicationCacheForWrite", ty: "*mut*const nsIApplicationCache" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetApplicationCacheForWrite",
                        params: &[Param { name: "aApplicationCacheForWrite", ty: "*const nsIApplicationCache" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

