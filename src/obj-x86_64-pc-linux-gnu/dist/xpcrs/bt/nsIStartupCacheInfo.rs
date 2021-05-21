//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/startupcache/nsIStartupCacheInfo.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStartupCacheInfo",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute boolean IgnoreDiskCache; */
                    Method {
                        name: "GetIgnoreDiskCache",
                        params: &[Param { name: "aIgnoreDiskCache", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean FoundDiskCacheOnInit; */
                    Method {
                        name: "GetFoundDiskCacheOnInit",
                        params: &[Param { name: "aFoundDiskCacheOnInit", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean WroteToDiskCache; */
                    Method {
                        name: "GetWroteToDiskCache",
                        params: &[Param { name: "aWroteToDiskCache", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AString DiskCachePath; */
                    Method {
                        name: "GetDiskCachePath",
                        params: &[Param { name: "aDiskCachePath", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

