//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIScriptedNotificationObserver.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgIScriptedNotificationObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void sizeAvailable (in imgIRequest aRequest); */
                    Method {
                        name: "SizeAvailable",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void frameUpdate (in imgIRequest aRequest); */
                    Method {
                        name: "FrameUpdate",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void frameComplete (in imgIRequest aRequest); */
                    Method {
                        name: "FrameComplete",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void loadComplete (in imgIRequest aRequest); */
                    Method {
                        name: "LoadComplete",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void decodeComplete (in imgIRequest aRequest); */
                    Method {
                        name: "DecodeComplete",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void discard (in imgIRequest aRequest); */
                    Method {
                        name: "Discard",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void isAnimated (in imgIRequest aRequest); */
                    Method {
                        name: "IsAnimated",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void hasTransparency (in imgIRequest aRequest); */
                    Method {
                        name: "HasTransparency",
                        params: &[Param { name: "aRequest", ty: "*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

