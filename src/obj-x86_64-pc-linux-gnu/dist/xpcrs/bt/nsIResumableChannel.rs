//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIResumableChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIResumableChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void resumeAt (in unsigned long long startPos, in ACString entityID); */
                    Method {
                        name: "ResumeAt",
                        params: &[Param { name: "startPos", ty: "u64" }, Param { name: "entityID", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString entityID; */
                    Method {
                        name: "GetEntityID",
                        params: &[Param { name: "aEntityID", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

