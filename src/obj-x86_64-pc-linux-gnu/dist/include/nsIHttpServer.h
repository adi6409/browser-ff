/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/test/httpserver/nsIHttpServer.idl
 */

#ifndef __gen_nsIHttpServer_h__
#define __gen_nsIHttpServer_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsIFile; /* forward declaration */

class nsIOutputStream; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */

class nsIHttpServer; /* forward declaration */

class nsIHttpServerStoppedCallback; /* forward declaration */

class nsIHttpRequestHandler; /* forward declaration */

class nsIHttpRequest; /* forward declaration */

class nsIHttpResponse; /* forward declaration */

class nsIHttpServerIdentity; /* forward declaration */


/* starting interface:    nsIHttpServer */
#define NS_IHTTPSERVER_IID_STR "cea8812e-faa6-4013-9396-f9936cbb74ec"

#define NS_IHTTPSERVER_IID \
  {0xcea8812e, 0xfaa6, 0x4013, \
    { 0x93, 0x96, 0xf9, 0x93, 0x6c, 0xbb, 0x74, 0xec }}

class NS_NO_VTABLE nsIHttpServer : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpServer;

  /* void start (in long port); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Start(int32_t port) = 0;

  /* void stop (in nsIHttpServerStoppedCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Stop(nsIHttpServerStoppedCallback *callback) = 0;

  /* void registerFile (in string path, in nsIFile file, [optional] in nsIHttpRequestHandler handler); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterFile(const char * path, nsIFile *file, nsIHttpRequestHandler *handler) = 0;

  /* void registerPathHandler (in string path, in nsIHttpRequestHandler handler); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterPathHandler(const char * path, nsIHttpRequestHandler *handler) = 0;

  /* void registerPrefixHandler (in string prefix, in nsIHttpRequestHandler handler); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterPrefixHandler(const char * prefix, nsIHttpRequestHandler *handler) = 0;

  /* void registerErrorHandler (in unsigned long code, in nsIHttpRequestHandler handler); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterErrorHandler(uint32_t code, nsIHttpRequestHandler *handler) = 0;

  /* void registerDirectory (in string path, in nsIFile dir); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterDirectory(const char * path, nsIFile *dir) = 0;

  /* void registerContentType (in string extension, in string type); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterContentType(const char * extension, const char * type) = 0;

  /* void setIndexHandler (in nsIHttpRequestHandler handler); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetIndexHandler(nsIHttpRequestHandler *handler) = 0;

  /* readonly attribute nsIHttpServerIdentity identity; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIdentity(nsIHttpServerIdentity **aIdentity) = 0;

  /* AString getState (in AString path, in AString key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetState(const nsAString& path, const nsAString& key, nsAString& _retval) = 0;

  /* void setState (in AString path, in AString key, in AString value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetState(const nsAString& path, const nsAString& key, const nsAString& value) = 0;

  /* AString getSharedState (in AString key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSharedState(const nsAString& key, nsAString& _retval) = 0;

  /* void setSharedState (in AString key, in AString value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSharedState(const nsAString& key, const nsAString& value) = 0;

  /* nsISupports getObjectState (in AString key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetObjectState(const nsAString& key, nsISupports **_retval) = 0;

  /* void setObjectState (in AString key, in nsISupports value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetObjectState(const nsAString& key, nsISupports *value) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpServer, NS_IHTTPSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPSERVER \
  NS_IMETHOD Start(int32_t port) override; \
  NS_IMETHOD Stop(nsIHttpServerStoppedCallback *callback) override; \
  NS_IMETHOD RegisterFile(const char * path, nsIFile *file, nsIHttpRequestHandler *handler) override; \
  NS_IMETHOD RegisterPathHandler(const char * path, nsIHttpRequestHandler *handler) override; \
  NS_IMETHOD RegisterPrefixHandler(const char * prefix, nsIHttpRequestHandler *handler) override; \
  NS_IMETHOD RegisterErrorHandler(uint32_t code, nsIHttpRequestHandler *handler) override; \
  NS_IMETHOD RegisterDirectory(const char * path, nsIFile *dir) override; \
  NS_IMETHOD RegisterContentType(const char * extension, const char * type) override; \
  NS_IMETHOD SetIndexHandler(nsIHttpRequestHandler *handler) override; \
  NS_IMETHOD GetIdentity(nsIHttpServerIdentity **aIdentity) override; \
  NS_IMETHOD GetState(const nsAString& path, const nsAString& key, nsAString& _retval) override; \
  NS_IMETHOD SetState(const nsAString& path, const nsAString& key, const nsAString& value) override; \
  NS_IMETHOD GetSharedState(const nsAString& key, nsAString& _retval) override; \
  NS_IMETHOD SetSharedState(const nsAString& key, const nsAString& value) override; \
  NS_IMETHOD GetObjectState(const nsAString& key, nsISupports **_retval) override; \
  NS_IMETHOD SetObjectState(const nsAString& key, nsISupports *value) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPSERVER \
  nsresult Start(int32_t port); \
  nsresult Stop(nsIHttpServerStoppedCallback *callback); \
  nsresult RegisterFile(const char * path, nsIFile *file, nsIHttpRequestHandler *handler); \
  nsresult RegisterPathHandler(const char * path, nsIHttpRequestHandler *handler); \
  nsresult RegisterPrefixHandler(const char * prefix, nsIHttpRequestHandler *handler); \
  nsresult RegisterErrorHandler(uint32_t code, nsIHttpRequestHandler *handler); \
  nsresult RegisterDirectory(const char * path, nsIFile *dir); \
  nsresult RegisterContentType(const char * extension, const char * type); \
  nsresult SetIndexHandler(nsIHttpRequestHandler *handler); \
  nsresult GetIdentity(nsIHttpServerIdentity **aIdentity); \
  nsresult GetState(const nsAString& path, const nsAString& key, nsAString& _retval); \
  nsresult SetState(const nsAString& path, const nsAString& key, const nsAString& value); \
  nsresult GetSharedState(const nsAString& key, nsAString& _retval); \
  nsresult SetSharedState(const nsAString& key, const nsAString& value); \
  nsresult GetObjectState(const nsAString& key, nsISupports **_retval); \
  nsresult SetObjectState(const nsAString& key, nsISupports *value); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPSERVER(_to) \
  NS_IMETHOD Start(int32_t port) override { return _to Start(port); } \
  NS_IMETHOD Stop(nsIHttpServerStoppedCallback *callback) override { return _to Stop(callback); } \
  NS_IMETHOD RegisterFile(const char * path, nsIFile *file, nsIHttpRequestHandler *handler) override { return _to RegisterFile(path, file, handler); } \
  NS_IMETHOD RegisterPathHandler(const char * path, nsIHttpRequestHandler *handler) override { return _to RegisterPathHandler(path, handler); } \
  NS_IMETHOD RegisterPrefixHandler(const char * prefix, nsIHttpRequestHandler *handler) override { return _to RegisterPrefixHandler(prefix, handler); } \
  NS_IMETHOD RegisterErrorHandler(uint32_t code, nsIHttpRequestHandler *handler) override { return _to RegisterErrorHandler(code, handler); } \
  NS_IMETHOD RegisterDirectory(const char * path, nsIFile *dir) override { return _to RegisterDirectory(path, dir); } \
  NS_IMETHOD RegisterContentType(const char * extension, const char * type) override { return _to RegisterContentType(extension, type); } \
  NS_IMETHOD SetIndexHandler(nsIHttpRequestHandler *handler) override { return _to SetIndexHandler(handler); } \
  NS_IMETHOD GetIdentity(nsIHttpServerIdentity **aIdentity) override { return _to GetIdentity(aIdentity); } \
  NS_IMETHOD GetState(const nsAString& path, const nsAString& key, nsAString& _retval) override { return _to GetState(path, key, _retval); } \
  NS_IMETHOD SetState(const nsAString& path, const nsAString& key, const nsAString& value) override { return _to SetState(path, key, value); } \
  NS_IMETHOD GetSharedState(const nsAString& key, nsAString& _retval) override { return _to GetSharedState(key, _retval); } \
  NS_IMETHOD SetSharedState(const nsAString& key, const nsAString& value) override { return _to SetSharedState(key, value); } \
  NS_IMETHOD GetObjectState(const nsAString& key, nsISupports **_retval) override { return _to GetObjectState(key, _retval); } \
  NS_IMETHOD SetObjectState(const nsAString& key, nsISupports *value) override { return _to SetObjectState(key, value); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPSERVER(_to) \
  NS_IMETHOD Start(int32_t port) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Start(port); } \
  NS_IMETHOD Stop(nsIHttpServerStoppedCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Stop(callback); } \
  NS_IMETHOD RegisterFile(const char * path, nsIFile *file, nsIHttpRequestHandler *handler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterFile(path, file, handler); } \
  NS_IMETHOD RegisterPathHandler(const char * path, nsIHttpRequestHandler *handler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterPathHandler(path, handler); } \
  NS_IMETHOD RegisterPrefixHandler(const char * prefix, nsIHttpRequestHandler *handler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterPrefixHandler(prefix, handler); } \
  NS_IMETHOD RegisterErrorHandler(uint32_t code, nsIHttpRequestHandler *handler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterErrorHandler(code, handler); } \
  NS_IMETHOD RegisterDirectory(const char * path, nsIFile *dir) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterDirectory(path, dir); } \
  NS_IMETHOD RegisterContentType(const char * extension, const char * type) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterContentType(extension, type); } \
  NS_IMETHOD SetIndexHandler(nsIHttpRequestHandler *handler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIndexHandler(handler); } \
  NS_IMETHOD GetIdentity(nsIHttpServerIdentity **aIdentity) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIdentity(aIdentity); } \
  NS_IMETHOD GetState(const nsAString& path, const nsAString& key, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetState(path, key, _retval); } \
  NS_IMETHOD SetState(const nsAString& path, const nsAString& key, const nsAString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetState(path, key, value); } \
  NS_IMETHOD GetSharedState(const nsAString& key, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSharedState(key, _retval); } \
  NS_IMETHOD SetSharedState(const nsAString& key, const nsAString& value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSharedState(key, value); } \
  NS_IMETHOD GetObjectState(const nsAString& key, nsISupports **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetObjectState(key, _retval); } \
  NS_IMETHOD SetObjectState(const nsAString& key, nsISupports *value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetObjectState(key, value); } 


/* starting interface:    nsIHttpServerStoppedCallback */
#define NS_IHTTPSERVERSTOPPEDCALLBACK_IID_STR "925a6d33-9937-4c63-abe1-a1c56a986455"

#define NS_IHTTPSERVERSTOPPEDCALLBACK_IID \
  {0x925a6d33, 0x9937, 0x4c63, \
    { 0xab, 0xe1, 0xa1, 0xc5, 0x6a, 0x98, 0x64, 0x55 }}

class NS_NO_VTABLE nsIHttpServerStoppedCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPSERVERSTOPPEDCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpServerStoppedCallback;

  /* void onStopped (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnStopped(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpServerStoppedCallback, NS_IHTTPSERVERSTOPPEDCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPSERVERSTOPPEDCALLBACK \
  NS_IMETHOD OnStopped(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPSERVERSTOPPEDCALLBACK \
  nsresult OnStopped(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPSERVERSTOPPEDCALLBACK(_to) \
  NS_IMETHOD OnStopped(void) override { return _to OnStopped(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPSERVERSTOPPEDCALLBACK(_to) \
  NS_IMETHOD OnStopped(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnStopped(); } 


/* starting interface:    nsIHttpServerIdentity */
#define NS_IHTTPSERVERIDENTITY_IID_STR "a89de175-ae8e-4c46-91a5-0dba99bbd284"

#define NS_IHTTPSERVERIDENTITY_IID \
  {0xa89de175, 0xae8e, 0x4c46, \
    { 0x91, 0xa5, 0x0d, 0xba, 0x99, 0xbb, 0xd2, 0x84 }}

class NS_NO_VTABLE nsIHttpServerIdentity : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPSERVERIDENTITY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpServerIdentity;

  /* readonly attribute string primaryScheme; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrimaryScheme(char * *aPrimaryScheme) = 0;

  /* readonly attribute string primaryHost; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrimaryHost(char * *aPrimaryHost) = 0;

  /* readonly attribute long primaryPort; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrimaryPort(int32_t *aPrimaryPort) = 0;

  /* void add (in string scheme, in string host, in long port); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Add(const char * scheme, const char * host, int32_t port) = 0;

  /* boolean remove (in string scheme, in string host, in long port); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Remove(const char * scheme, const char * host, int32_t port, bool *_retval) = 0;

  /* boolean has (in string scheme, in string host, in long port); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Has(const char * scheme, const char * host, int32_t port, bool *_retval) = 0;

  /* string getScheme (in string host, in long port); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetScheme(const char * host, int32_t port, char * *_retval) = 0;

  /* void setPrimary (in string scheme, in string host, in long port); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPrimary(const char * scheme, const char * host, int32_t port) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpServerIdentity, NS_IHTTPSERVERIDENTITY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPSERVERIDENTITY \
  NS_IMETHOD GetPrimaryScheme(char * *aPrimaryScheme) override; \
  NS_IMETHOD GetPrimaryHost(char * *aPrimaryHost) override; \
  NS_IMETHOD GetPrimaryPort(int32_t *aPrimaryPort) override; \
  NS_IMETHOD Add(const char * scheme, const char * host, int32_t port) override; \
  NS_IMETHOD Remove(const char * scheme, const char * host, int32_t port, bool *_retval) override; \
  NS_IMETHOD Has(const char * scheme, const char * host, int32_t port, bool *_retval) override; \
  NS_IMETHOD GetScheme(const char * host, int32_t port, char * *_retval) override; \
  NS_IMETHOD SetPrimary(const char * scheme, const char * host, int32_t port) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPSERVERIDENTITY \
  nsresult GetPrimaryScheme(char * *aPrimaryScheme); \
  nsresult GetPrimaryHost(char * *aPrimaryHost); \
  nsresult GetPrimaryPort(int32_t *aPrimaryPort); \
  nsresult Add(const char * scheme, const char * host, int32_t port); \
  nsresult Remove(const char * scheme, const char * host, int32_t port, bool *_retval); \
  nsresult Has(const char * scheme, const char * host, int32_t port, bool *_retval); \
  nsresult GetScheme(const char * host, int32_t port, char * *_retval); \
  nsresult SetPrimary(const char * scheme, const char * host, int32_t port); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPSERVERIDENTITY(_to) \
  NS_IMETHOD GetPrimaryScheme(char * *aPrimaryScheme) override { return _to GetPrimaryScheme(aPrimaryScheme); } \
  NS_IMETHOD GetPrimaryHost(char * *aPrimaryHost) override { return _to GetPrimaryHost(aPrimaryHost); } \
  NS_IMETHOD GetPrimaryPort(int32_t *aPrimaryPort) override { return _to GetPrimaryPort(aPrimaryPort); } \
  NS_IMETHOD Add(const char * scheme, const char * host, int32_t port) override { return _to Add(scheme, host, port); } \
  NS_IMETHOD Remove(const char * scheme, const char * host, int32_t port, bool *_retval) override { return _to Remove(scheme, host, port, _retval); } \
  NS_IMETHOD Has(const char * scheme, const char * host, int32_t port, bool *_retval) override { return _to Has(scheme, host, port, _retval); } \
  NS_IMETHOD GetScheme(const char * host, int32_t port, char * *_retval) override { return _to GetScheme(host, port, _retval); } \
  NS_IMETHOD SetPrimary(const char * scheme, const char * host, int32_t port) override { return _to SetPrimary(scheme, host, port); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPSERVERIDENTITY(_to) \
  NS_IMETHOD GetPrimaryScheme(char * *aPrimaryScheme) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryScheme(aPrimaryScheme); } \
  NS_IMETHOD GetPrimaryHost(char * *aPrimaryHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryHost(aPrimaryHost); } \
  NS_IMETHOD GetPrimaryPort(int32_t *aPrimaryPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrimaryPort(aPrimaryPort); } \
  NS_IMETHOD Add(const char * scheme, const char * host, int32_t port) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Add(scheme, host, port); } \
  NS_IMETHOD Remove(const char * scheme, const char * host, int32_t port, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Remove(scheme, host, port, _retval); } \
  NS_IMETHOD Has(const char * scheme, const char * host, int32_t port, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Has(scheme, host, port, _retval); } \
  NS_IMETHOD GetScheme(const char * host, int32_t port, char * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScheme(host, port, _retval); } \
  NS_IMETHOD SetPrimary(const char * scheme, const char * host, int32_t port) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrimary(scheme, host, port); } 


/* starting interface:    nsIHttpRequestHandler */
#define NS_IHTTPREQUESTHANDLER_IID_STR "2bbb4db7-d285-42b3-a3ce-142b8cc7e139"

#define NS_IHTTPREQUESTHANDLER_IID \
  {0x2bbb4db7, 0xd285, 0x42b3, \
    { 0xa3, 0xce, 0x14, 0x2b, 0x8c, 0xc7, 0xe1, 0x39 }}

class NS_NO_VTABLE nsIHttpRequestHandler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPREQUESTHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpRequestHandler;

  /* void handle (in nsIHttpRequest request, in nsIHttpResponse response); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Handle(nsIHttpRequest *request, nsIHttpResponse *response) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpRequestHandler, NS_IHTTPREQUESTHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPREQUESTHANDLER \
  NS_IMETHOD Handle(nsIHttpRequest *request, nsIHttpResponse *response) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPREQUESTHANDLER \
  nsresult Handle(nsIHttpRequest *request, nsIHttpResponse *response); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPREQUESTHANDLER(_to) \
  NS_IMETHOD Handle(nsIHttpRequest *request, nsIHttpResponse *response) override { return _to Handle(request, response); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPREQUESTHANDLER(_to) \
  NS_IMETHOD Handle(nsIHttpRequest *request, nsIHttpResponse *response) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Handle(request, response); } 


/* starting interface:    nsIHttpRequest */
#define NS_IHTTPREQUEST_IID_STR "978cf30e-ad73-42ee-8f22-fe0aaf1bf5d2"

#define NS_IHTTPREQUEST_IID \
  {0x978cf30e, 0xad73, 0x42ee, \
    { 0x8f, 0x22, 0xfe, 0x0a, 0xaf, 0x1b, 0xf5, 0xd2 }}

class NS_NO_VTABLE nsIHttpRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpRequest;

  /* readonly attribute string method; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMethod(char * *aMethod) = 0;

  /* readonly attribute string scheme; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetScheme(char * *aScheme) = 0;

  /* readonly attribute string host; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHost(char * *aHost) = 0;

  /* readonly attribute unsigned long port; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPort(uint32_t *aPort) = 0;

  /* readonly attribute string path; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPath(char * *aPath) = 0;

  /* readonly attribute string queryString; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetQueryString(char * *aQueryString) = 0;

  /* readonly attribute string httpVersion; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHttpVersion(char * *aHttpVersion) = 0;

  /* string getHeader (in string fieldName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHeader(const char * fieldName, char * *_retval) = 0;

  /* boolean hasHeader (in string fieldName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasHeader(const char * fieldName, bool *_retval) = 0;

  /* readonly attribute nsISimpleEnumerator headers; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHeaders(nsISimpleEnumerator **aHeaders) = 0;

  /* readonly attribute nsIInputStream bodyInputStream; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBodyInputStream(nsIInputStream **aBodyInputStream) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpRequest, NS_IHTTPREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPREQUEST \
  NS_IMETHOD GetMethod(char * *aMethod) override; \
  NS_IMETHOD GetScheme(char * *aScheme) override; \
  NS_IMETHOD GetHost(char * *aHost) override; \
  NS_IMETHOD GetPort(uint32_t *aPort) override; \
  NS_IMETHOD GetPath(char * *aPath) override; \
  NS_IMETHOD GetQueryString(char * *aQueryString) override; \
  NS_IMETHOD GetHttpVersion(char * *aHttpVersion) override; \
  NS_IMETHOD GetHeader(const char * fieldName, char * *_retval) override; \
  NS_IMETHOD HasHeader(const char * fieldName, bool *_retval) override; \
  NS_IMETHOD GetHeaders(nsISimpleEnumerator **aHeaders) override; \
  NS_IMETHOD GetBodyInputStream(nsIInputStream **aBodyInputStream) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPREQUEST \
  nsresult GetMethod(char * *aMethod); \
  nsresult GetScheme(char * *aScheme); \
  nsresult GetHost(char * *aHost); \
  nsresult GetPort(uint32_t *aPort); \
  nsresult GetPath(char * *aPath); \
  nsresult GetQueryString(char * *aQueryString); \
  nsresult GetHttpVersion(char * *aHttpVersion); \
  nsresult GetHeader(const char * fieldName, char * *_retval); \
  nsresult HasHeader(const char * fieldName, bool *_retval); \
  nsresult GetHeaders(nsISimpleEnumerator **aHeaders); \
  nsresult GetBodyInputStream(nsIInputStream **aBodyInputStream); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPREQUEST(_to) \
  NS_IMETHOD GetMethod(char * *aMethod) override { return _to GetMethod(aMethod); } \
  NS_IMETHOD GetScheme(char * *aScheme) override { return _to GetScheme(aScheme); } \
  NS_IMETHOD GetHost(char * *aHost) override { return _to GetHost(aHost); } \
  NS_IMETHOD GetPort(uint32_t *aPort) override { return _to GetPort(aPort); } \
  NS_IMETHOD GetPath(char * *aPath) override { return _to GetPath(aPath); } \
  NS_IMETHOD GetQueryString(char * *aQueryString) override { return _to GetQueryString(aQueryString); } \
  NS_IMETHOD GetHttpVersion(char * *aHttpVersion) override { return _to GetHttpVersion(aHttpVersion); } \
  NS_IMETHOD GetHeader(const char * fieldName, char * *_retval) override { return _to GetHeader(fieldName, _retval); } \
  NS_IMETHOD HasHeader(const char * fieldName, bool *_retval) override { return _to HasHeader(fieldName, _retval); } \
  NS_IMETHOD GetHeaders(nsISimpleEnumerator **aHeaders) override { return _to GetHeaders(aHeaders); } \
  NS_IMETHOD GetBodyInputStream(nsIInputStream **aBodyInputStream) override { return _to GetBodyInputStream(aBodyInputStream); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPREQUEST(_to) \
  NS_IMETHOD GetMethod(char * *aMethod) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMethod(aMethod); } \
  NS_IMETHOD GetScheme(char * *aScheme) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScheme(aScheme); } \
  NS_IMETHOD GetHost(char * *aHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHost(aHost); } \
  NS_IMETHOD GetPort(uint32_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } \
  NS_IMETHOD GetPath(char * *aPath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPath(aPath); } \
  NS_IMETHOD GetQueryString(char * *aQueryString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetQueryString(aQueryString); } \
  NS_IMETHOD GetHttpVersion(char * *aHttpVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHttpVersion(aHttpVersion); } \
  NS_IMETHOD GetHeader(const char * fieldName, char * *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHeader(fieldName, _retval); } \
  NS_IMETHOD HasHeader(const char * fieldName, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasHeader(fieldName, _retval); } \
  NS_IMETHOD GetHeaders(nsISimpleEnumerator **aHeaders) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHeaders(aHeaders); } \
  NS_IMETHOD GetBodyInputStream(nsIInputStream **aBodyInputStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBodyInputStream(aBodyInputStream); } 


/* starting interface:    nsIHttpResponse */
#define NS_IHTTPRESPONSE_IID_STR "1acd16c2-dc59-42fa-9160-4f26c43c1c21"

#define NS_IHTTPRESPONSE_IID \
  {0x1acd16c2, 0xdc59, 0x42fa, \
    { 0x91, 0x60, 0x4f, 0x26, 0xc4, 0x3c, 0x1c, 0x21 }}

class NS_NO_VTABLE nsIHttpResponse : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHTTPRESPONSE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHttpResponse;

  /* void setStatusLine (in string httpVersion, in unsigned short statusCode, in string description); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetStatusLine(const char * httpVersion, uint16_t statusCode, const char * description) = 0;

  /* void setHeader (in string name, in string value, in boolean merge); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetHeader(const char * name, const char * value, bool merge) = 0;

  /* void setHeaderNoCheck (in string name, in string value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetHeaderNoCheck(const char * name, const char * value) = 0;

  /* readonly attribute nsIOutputStream bodyOutputStream; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBodyOutputStream(nsIOutputStream **aBodyOutputStream) = 0;

  /* void write (in string data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Write(const char * data) = 0;

  /* void processAsync (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ProcessAsync(void) = 0;

  /* void seizePower (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SeizePower(void) = 0;

  /* void finish (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Finish(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHttpResponse, NS_IHTTPRESPONSE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHTTPRESPONSE \
  NS_IMETHOD SetStatusLine(const char * httpVersion, uint16_t statusCode, const char * description) override; \
  NS_IMETHOD SetHeader(const char * name, const char * value, bool merge) override; \
  NS_IMETHOD SetHeaderNoCheck(const char * name, const char * value) override; \
  NS_IMETHOD GetBodyOutputStream(nsIOutputStream **aBodyOutputStream) override; \
  NS_IMETHOD Write(const char * data) override; \
  NS_IMETHOD ProcessAsync(void) override; \
  NS_IMETHOD SeizePower(void) override; \
  NS_IMETHOD Finish(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHTTPRESPONSE \
  nsresult SetStatusLine(const char * httpVersion, uint16_t statusCode, const char * description); \
  nsresult SetHeader(const char * name, const char * value, bool merge); \
  nsresult SetHeaderNoCheck(const char * name, const char * value); \
  nsresult GetBodyOutputStream(nsIOutputStream **aBodyOutputStream); \
  nsresult Write(const char * data); \
  nsresult ProcessAsync(void); \
  nsresult SeizePower(void); \
  nsresult Finish(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHTTPRESPONSE(_to) \
  NS_IMETHOD SetStatusLine(const char * httpVersion, uint16_t statusCode, const char * description) override { return _to SetStatusLine(httpVersion, statusCode, description); } \
  NS_IMETHOD SetHeader(const char * name, const char * value, bool merge) override { return _to SetHeader(name, value, merge); } \
  NS_IMETHOD SetHeaderNoCheck(const char * name, const char * value) override { return _to SetHeaderNoCheck(name, value); } \
  NS_IMETHOD GetBodyOutputStream(nsIOutputStream **aBodyOutputStream) override { return _to GetBodyOutputStream(aBodyOutputStream); } \
  NS_IMETHOD Write(const char * data) override { return _to Write(data); } \
  NS_IMETHOD ProcessAsync(void) override { return _to ProcessAsync(); } \
  NS_IMETHOD SeizePower(void) override { return _to SeizePower(); } \
  NS_IMETHOD Finish(void) override { return _to Finish(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHTTPRESPONSE(_to) \
  NS_IMETHOD SetStatusLine(const char * httpVersion, uint16_t statusCode, const char * description) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetStatusLine(httpVersion, statusCode, description); } \
  NS_IMETHOD SetHeader(const char * name, const char * value, bool merge) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHeader(name, value, merge); } \
  NS_IMETHOD SetHeaderNoCheck(const char * name, const char * value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHeaderNoCheck(name, value); } \
  NS_IMETHOD GetBodyOutputStream(nsIOutputStream **aBodyOutputStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBodyOutputStream(aBodyOutputStream); } \
  NS_IMETHOD Write(const char * data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Write(data); } \
  NS_IMETHOD ProcessAsync(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProcessAsync(); } \
  NS_IMETHOD SeizePower(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SeizePower(); } \
  NS_IMETHOD Finish(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Finish(); } 


#endif /* __gen_nsIHttpServer_h__ */
