//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/test/httpserver/nsIHttpServer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIHttpServer",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void start (in long port); */
                    Method {
                        name: "Start",
                        params: &[Param { name: "port", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void stop (in nsIHttpServerStoppedCallback callback); */
                    Method {
                        name: "Stop",
                        params: &[Param { name: "callback", ty: "*const nsIHttpServerStoppedCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerFile (in string path, in nsIFile file, [optional] in nsIHttpRequestHandler handler); */
                    Method {
                        name: "RegisterFile",
                        params: &[Param { name: "path", ty: "*const libc::c_char" }, Param { name: "file", ty: "*const nsIFile" }, Param { name: "handler", ty: "*const nsIHttpRequestHandler" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerPathHandler (in string path, in nsIHttpRequestHandler handler); */
                    Method {
                        name: "RegisterPathHandler",
                        params: &[Param { name: "path", ty: "*const libc::c_char" }, Param { name: "handler", ty: "*const nsIHttpRequestHandler" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerPrefixHandler (in string prefix, in nsIHttpRequestHandler handler); */
                    Method {
                        name: "RegisterPrefixHandler",
                        params: &[Param { name: "prefix", ty: "*const libc::c_char" }, Param { name: "handler", ty: "*const nsIHttpRequestHandler" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerErrorHandler (in unsigned long code, in nsIHttpRequestHandler handler); */
                    Method {
                        name: "RegisterErrorHandler",
                        params: &[Param { name: "code", ty: "u32" }, Param { name: "handler", ty: "*const nsIHttpRequestHandler" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerDirectory (in string path, in nsIFile dir); */
                    Method {
                        name: "RegisterDirectory",
                        params: &[Param { name: "path", ty: "*const libc::c_char" }, Param { name: "dir", ty: "*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void registerContentType (in string extension, in string type); */
                    Method {
                        name: "RegisterContentType",
                        params: &[Param { name: "extension", ty: "*const libc::c_char" }, Param { name: "type_", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setIndexHandler (in nsIHttpRequestHandler handler); */
                    Method {
                        name: "SetIndexHandler",
                        params: &[Param { name: "handler", ty: "*const nsIHttpRequestHandler" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIHttpServerIdentity identity; */
                    Method {
                        name: "GetIdentity",
                        params: &[Param { name: "aIdentity", ty: "*mut*const nsIHttpServerIdentity" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getState (in AString path, in AString key); */
                    Method {
                        name: "GetState",
                        params: &[Param { name: "path", ty: "*const ::nsstring::nsAString" }, Param { name: "key", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setState (in AString path, in AString key, in AString value); */
                    Method {
                        name: "SetState",
                        params: &[Param { name: "path", ty: "*const ::nsstring::nsAString" }, Param { name: "key", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AString getSharedState (in AString key); */
                    Method {
                        name: "GetSharedState",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setSharedState (in AString key, in AString value); */
                    Method {
                        name: "SetSharedState",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsISupports getObjectState (in AString key); */
                    Method {
                        name: "GetObjectState",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsAString" }, Param { name: "_retval", ty: "*mut *const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setObjectState (in AString key, in nsISupports value); */
                    Method {
                        name: "SetObjectState",
                        params: &[Param { name: "key", ty: "*const ::nsstring::nsAString" }, Param { name: "value", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIHttpServerStoppedCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onStopped (); */
                    Method {
                        name: "OnStopped",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIHttpServerIdentity",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute string primaryScheme; */
                    Method {
                        name: "GetPrimaryScheme",
                        params: &[Param { name: "aPrimaryScheme", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute string primaryHost; */
                    Method {
                        name: "GetPrimaryHost",
                        params: &[Param { name: "aPrimaryHost", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long primaryPort; */
                    Method {
                        name: "GetPrimaryPort",
                        params: &[Param { name: "aPrimaryPort", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void add (in string scheme, in string host, in long port); */
                    Method {
                        name: "Add",
                        params: &[Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "host", ty: "*const libc::c_char" }, Param { name: "port", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean remove (in string scheme, in string host, in long port); */
                    Method {
                        name: "Remove",
                        params: &[Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "host", ty: "*const libc::c_char" }, Param { name: "port", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean has (in string scheme, in string host, in long port); */
                    Method {
                        name: "Has",
                        params: &[Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "host", ty: "*const libc::c_char" }, Param { name: "port", ty: "i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string getScheme (in string host, in long port); */
                    Method {
                        name: "GetScheme",
                        params: &[Param { name: "host", ty: "*const libc::c_char" }, Param { name: "port", ty: "i32" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setPrimary (in string scheme, in string host, in long port); */
                    Method {
                        name: "SetPrimary",
                        params: &[Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "host", ty: "*const libc::c_char" }, Param { name: "port", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIHttpRequestHandler",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handle (in nsIHttpRequest request, in nsIHttpResponse response); */
                    Method {
                        name: "Handle",
                        params: &[Param { name: "request", ty: "*const nsIHttpRequest" }, Param { name: "response", ty: "*const nsIHttpResponse" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIHttpRequest",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute string method; */
                    Method {
                        name: "GetMethod",
                        params: &[Param { name: "aMethod", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute string scheme; */
                    Method {
                        name: "GetScheme",
                        params: &[Param { name: "aScheme", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute string host; */
                    Method {
                        name: "GetHost",
                        params: &[Param { name: "aHost", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long port; */
                    Method {
                        name: "GetPort",
                        params: &[Param { name: "aPort", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute string path; */
                    Method {
                        name: "GetPath",
                        params: &[Param { name: "aPath", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute string queryString; */
                    Method {
                        name: "GetQueryString",
                        params: &[Param { name: "aQueryString", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute string httpVersion; */
                    Method {
                        name: "GetHttpVersion",
                        params: &[Param { name: "aHttpVersion", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string getHeader (in string fieldName); */
                    Method {
                        name: "GetHeader",
                        params: &[Param { name: "fieldName", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean hasHeader (in string fieldName); */
                    Method {
                        name: "HasHeader",
                        params: &[Param { name: "fieldName", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsISimpleEnumerator headers; */
                    Method {
                        name: "GetHeaders",
                        params: &[Param { name: "aHeaders", ty: "*mut*const nsISimpleEnumerator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIInputStream bodyInputStream; */
                    Method {
                        name: "GetBodyInputStream",
                        params: &[Param { name: "aBodyInputStream", ty: "*mut*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIHttpResponse",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void setStatusLine (in string httpVersion, in unsigned short statusCode, in string description); */
                    Method {
                        name: "SetStatusLine",
                        params: &[Param { name: "httpVersion", ty: "*const libc::c_char" }, Param { name: "statusCode", ty: "u16" }, Param { name: "description", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setHeader (in string name, in string value, in boolean merge); */
                    Method {
                        name: "SetHeader",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }, Param { name: "merge", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void setHeaderNoCheck (in string name, in string value); */
                    Method {
                        name: "SetHeaderNoCheck",
                        params: &[Param { name: "name", ty: "*const libc::c_char" }, Param { name: "value", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIOutputStream bodyOutputStream; */
                    Method {
                        name: "GetBodyOutputStream",
                        params: &[Param { name: "aBodyOutputStream", ty: "*mut*const nsIOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void write (in string data); */
                    Method {
                        name: "Write",
                        params: &[Param { name: "data", ty: "*const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void processAsync (); */
                    Method {
                        name: "ProcessAsync",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void seizePower (); */
                    Method {
                        name: "SeizePower",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void finish (); */
                    Method {
                        name: "Finish",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

