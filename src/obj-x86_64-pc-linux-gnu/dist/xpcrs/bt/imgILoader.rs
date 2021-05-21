//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgILoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "imgILoader",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* imgIRequest loadImageXPCOM (in nsIURI aURI, in nsIURI aInitialDocumentURL, in nsIReferrerInfo aReferrerInfo, in nsIPrincipal aLoadingPrincipal, in nsILoadGroup aLoadGroup, in imgINotificationObserver aObserver, in Document aLoadingDocument, in nsLoadFlags aLoadFlags, in nsISupports cacheKey, [optional] in nsContentPolicyType aContentPolicyType); */
                    Method {
                        name: "LoadImageXPCOM",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aInitialDocumentURL", ty: "*const nsIURI" }, Param { name: "aReferrerInfo", ty: "*const nsIReferrerInfo" }, Param { name: "aLoadingPrincipal", ty: "*const nsIPrincipal" }, Param { name: "aLoadGroup", ty: "*const nsILoadGroup" }, Param { name: "aObserver", ty: "*const imgINotificationObserver" }, Param { name: "aLoadingDocument", ty: "*const libc::c_void" }, Param { name: "aLoadFlags", ty: "nsLoadFlags" }, Param { name: "cacheKey", ty: "*const nsISupports" }, Param { name: "aContentPolicyType", ty: "nsContentPolicyType" }, Param { name: "_retval", ty: "*mut*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    /* imgIRequest loadImageWithChannelXPCOM (in nsIChannel aChannel, in imgINotificationObserver aObserver, in Document aLoadingDocument, out nsIStreamListener aListener); */
                    Method {
                        name: "LoadImageWithChannelXPCOM",
                        params: &[Param { name: "aChannel", ty: "*const nsIChannel" }, Param { name: "aObserver", ty: "*const imgINotificationObserver" }, Param { name: "aLoadingDocument", ty: "*const libc::c_void" }, Param { name: "aListener", ty: "*mut*const nsIStreamListener" }, Param { name: "_retval", ty: "*mut*const imgIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

