//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIRequest.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgIRequest",
            base: Some("nsIRequest"),
            methods: Ok(&[
                    /* readonly attribute imgIContainer image; */
                    Method {
                        name: "GetImage",
                        params: &[Param { name: "aImage", ty: "*mut *const imgIContainer" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [infallible] readonly attribute unsigned long producerId; */
                    Method {
                        name: "GetProducerId",
                        params: &[Param { name: "aProducerId", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long imageStatus; */
                    Method {
                        name: "GetImageStatus",
                        params: &[Param { name: "aImageStatus", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] readonly attribute nsresult imageErrorCode; */
                    Method {
                        name: "GetImageErrorCode",
                        params: &[Param { name: "aImageErrorCode", ty: "*mut ::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI URI; */
                    Method {
                        name: "GetURI",
                        params: &[Param { name: "aURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIURI finalURI; */
                    Method {
                        name: "GetFinalURI",
                        params: &[Param { name: "aFinalURI", ty: "*mut*const nsIURI" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute imgINotificationObserver notificationObserver; */
                    Method {
                        name: "GetNotificationObserver",
                        params: &[Param { name: "aNotificationObserver", ty: "*mut*const imgINotificationObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute string mimeType; */
                    Method {
                        name: "GetMimeType",
                        params: &[Param { name: "aMimeType", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* imgIRequest clone (in imgINotificationObserver aObserver); */
                    Method {
                        name: "Clone",
                        params: &[Param { name: "aObserver", ty: "*const imgINotificationObserver" }, Param { name: "_retval", ty: "*mut *const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIPrincipal imagePrincipal; */
                    Method {
                        name: "GetImagePrincipal",
                        params: &[Param { name: "aImagePrincipal", ty: "*mut*const nsIPrincipal" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool hadCrossOriginRedirects; */
                    Method {
                        name: "GetHadCrossOriginRedirects",
                        params: &[Param { name: "aHadCrossOriginRedirects", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute bool multipart; */
                    Method {
                        name: "GetMultipart",
                        params: &[Param { name: "aMultipart", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long CORSMode; */
                    Method {
                        name: "GetCORSMode",
                        params: &[Param { name: "aCORSMode", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void cancelAndForgetObserver (in nsresult aStatus); */
                    Method {
                        name: "CancelAndForgetObserver",
                        params: &[Param { name: "aStatus", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void startDecoding (in uint32_t aFlags); */
                    Method {
                        name: "StartDecoding",
                        params: &[Param { name: "aFlags", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript,notxpcom] boolean startDecodingWithResult (in uint32_t aFlags); */
                    Method {
                        name: "StartDecodingWithResult",
                        params: &[Param { name: "aFlags", ty: "uint32_t" }],
                        ret: "bool",
                    },

                    /* [noscript,notxpcom] imgIContainer_DecodeResult requestDecodeWithResult (in uint32_t aFlags); */
                    Method {
                        name: "RequestDecodeWithResult",
                        params: &[Param { name: "aFlags", ty: "uint32_t" }],
                        ret: "u8",
                    },

                    /* void lockImage (); */
                    Method {
                        name: "LockImage",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void unlockImage (); */
                    Method {
                        name: "UnlockImage",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void requestDiscard (); */
                    Method {
                        name: "RequestDiscard",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* imgIRequest getStaticRequest (); */
                    Method {
                        name: "GetStaticRequest",
                        params: &[Param { name: "_retval", ty: "*mut *const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void incrementAnimationConsumers (); */
                    Method {
                        name: "IncrementAnimationConsumers",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void decrementAnimationConsumers (); */
                    Method {
                        name: "DecrementAnimationConsumers",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void boostPriority (in uint32_t aCategory); */
                    Method {
                        name: "BoostPriority",
                        params: &[Param { name: "aCategory", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

