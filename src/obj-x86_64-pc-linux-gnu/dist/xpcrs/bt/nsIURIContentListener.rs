//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIURIContentListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURIContentListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean doContent (in ACString aContentType, in boolean aIsContentPreferred, in nsIRequest aRequest, out nsIStreamListener aContentHandler); */
                    Method {
                        name: "DoContent",
                        params: &[Param { name: "aContentType", ty: "*const ::nsstring::nsACString" }, Param { name: "aIsContentPreferred", ty: "bool" }, Param { name: "aRequest", ty: "*const nsIRequest" }, Param { name: "aContentHandler", ty: "*mut*const nsIStreamListener" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean isPreferred (in string aContentType, out string aDesiredContentType); */
                    Method {
                        name: "IsPreferred",
                        params: &[Param { name: "aContentType", ty: "*const libc::c_char" }, Param { name: "aDesiredContentType", ty: "*mut *const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean canHandleContent (in string aContentType, in boolean aIsContentPreferred, out string aDesiredContentType); */
                    Method {
                        name: "CanHandleContent",
                        params: &[Param { name: "aContentType", ty: "*const libc::c_char" }, Param { name: "aIsContentPreferred", ty: "bool" }, Param { name: "aDesiredContentType", ty: "*mut *const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsISupports loadCookie; */
                    Method {
                        name: "GetLoadCookie",
                        params: &[Param { name: "aLoadCookie", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetLoadCookie",
                        params: &[Param { name: "aLoadCookie", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIURIContentListener parentContentListener; */
                    Method {
                        name: "GetParentContentListener",
                        params: &[Param { name: "aParentContentListener", ty: "*mut *const nsIURIContentListener" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetParentContentListener",
                        params: &[Param { name: "aParentContentListener", ty: "*const nsIURIContentListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

