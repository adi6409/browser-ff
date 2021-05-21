//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIContentPolicy.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIContentPolicy",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* short shouldLoad (in nsIURI aContentLocation, in nsILoadInfo aLoadInfo, in ACString aMimeTypeGuess); */
                    Method {
                        name: "ShouldLoad",
                        params: &[Param { name: "aContentLocation", ty: "*const nsIURI" }, Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }, Param { name: "aMimeTypeGuess", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    /* short shouldProcess (in nsIURI aContentLocation, in nsILoadInfo aLoadInfo, in ACString aMimeType); */
                    Method {
                        name: "ShouldProcess",
                        params: &[Param { name: "aContentLocation", ty: "*const nsIURI" }, Param { name: "aLoadInfo", ty: "*const nsILoadInfo" }, Param { name: "aMimeType", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut i16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

