/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRequestContext.idl
 */

#ifndef __gen_nsIRequestContext_h__
#define __gen_nsIRequestContext_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
// Forward-declare mozilla::net::SpdyPushCache
namespace mozilla {
namespace net {
class SpdyPushCache;
}
}
class nsILoadGroup; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsIStreamListener; /* forward declaration */


/* starting interface:    nsIRequestTailUnblockCallback */
#define NS_IREQUESTTAILUNBLOCKCALLBACK_IID_STR "7eb361d4-37a5-42c9-afae-f6c88fe7c394"

#define NS_IREQUESTTAILUNBLOCKCALLBACK_IID \
  {0x7eb361d4, 0x37a5, 0x42c9, \
    { 0xaf, 0xae, 0xf6, 0xc8, 0x8f, 0xe7, 0xc3, 0x94 }}

class NS_NO_VTABLE nsIRequestTailUnblockCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREQUESTTAILUNBLOCKCALLBACK_IID)

  /* void onTailUnblock (in nsresult aResult); */
  NS_IMETHOD OnTailUnblock(nsresult aResult) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRequestTailUnblockCallback, NS_IREQUESTTAILUNBLOCKCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREQUESTTAILUNBLOCKCALLBACK \
  NS_IMETHOD OnTailUnblock(nsresult aResult) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREQUESTTAILUNBLOCKCALLBACK \
  nsresult OnTailUnblock(nsresult aResult); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREQUESTTAILUNBLOCKCALLBACK(_to) \
  NS_IMETHOD OnTailUnblock(nsresult aResult) override { return _to OnTailUnblock(aResult); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREQUESTTAILUNBLOCKCALLBACK(_to) \
  NS_IMETHOD OnTailUnblock(nsresult aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnTailUnblock(aResult); } 


/* starting interface:    nsIRequestContext */
#define NS_IREQUESTCONTEXT_IID_STR "658e3e6e-8633-4b1a-8d66-fa9f72293e63"

#define NS_IREQUESTCONTEXT_IID \
  {0x658e3e6e, 0x8633, 0x4b1a, \
    { 0x8d, 0x66, 0xfa, 0x9f, 0x72, 0x29, 0x3e, 0x63 }}

class NS_NO_VTABLE nsIRequestContext : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREQUESTCONTEXT_IID)

  /* [nostdcall,notxpcom] readonly attribute unsigned long long ID; */
  virtual uint64_t GetID() = 0;

  /* void beginLoad (); */
  NS_IMETHOD BeginLoad(void) = 0;

  /* void DOMContentLoaded (); */
  NS_IMETHOD DOMContentLoaded(void) = 0;

  /* readonly attribute unsigned long blockingTransactionCount; */
  NS_IMETHOD GetBlockingTransactionCount(uint32_t *aBlockingTransactionCount) = 0;

  /* void addBlockingTransaction (); */
  NS_IMETHOD AddBlockingTransaction(void) = 0;

  /* unsigned long removeBlockingTransaction (); */
  NS_IMETHOD RemoveBlockingTransaction(uint32_t *_retval) = 0;

  /* [nostdcall,notxpcom] attribute SpdyPushCachePtr spdyPushCache; */
  virtual mozilla::net::SpdyPushCache * GetSpdyPushCache() = 0;
  virtual void SetSpdyPushCache(mozilla::net::SpdyPushCache * aSpdyPushCache) = 0;

  /* void addNonTailRequest (); */
  NS_IMETHOD AddNonTailRequest(void) = 0;

  /* void removeNonTailRequest (); */
  NS_IMETHOD RemoveNonTailRequest(void) = 0;

  /* [must_use] boolean isContextTailBlocked (in nsIRequestTailUnblockCallback callback); */
  [[nodiscard]] NS_IMETHOD IsContextTailBlocked(nsIRequestTailUnblockCallback *callback, bool *_retval) = 0;

  /* void cancelTailedRequest (in nsIRequestTailUnblockCallback request); */
  NS_IMETHOD CancelTailedRequest(nsIRequestTailUnblockCallback *request) = 0;

  /* void cancelTailPendingRequests (in nsresult aResult); */
  NS_IMETHOD CancelTailPendingRequests(nsresult aResult) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRequestContext, NS_IREQUESTCONTEXT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREQUESTCONTEXT \
  virtual uint64_t GetID() override; \
  NS_IMETHOD BeginLoad(void) override; \
  NS_IMETHOD DOMContentLoaded(void) override; \
  NS_IMETHOD GetBlockingTransactionCount(uint32_t *aBlockingTransactionCount) override; \
  NS_IMETHOD AddBlockingTransaction(void) override; \
  NS_IMETHOD RemoveBlockingTransaction(uint32_t *_retval) override; \
  virtual mozilla::net::SpdyPushCache * GetSpdyPushCache() override; \
  virtual void SetSpdyPushCache(mozilla::net::SpdyPushCache * aSpdyPushCache) override; \
  NS_IMETHOD AddNonTailRequest(void) override; \
  NS_IMETHOD RemoveNonTailRequest(void) override; \
  [[nodiscard]] NS_IMETHOD IsContextTailBlocked(nsIRequestTailUnblockCallback *callback, bool *_retval) override; \
  NS_IMETHOD CancelTailedRequest(nsIRequestTailUnblockCallback *request) override; \
  NS_IMETHOD CancelTailPendingRequests(nsresult aResult) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREQUESTCONTEXT \
  uint64_t GetID(); \
  nsresult BeginLoad(void); \
  nsresult DOMContentLoaded(void); \
  nsresult GetBlockingTransactionCount(uint32_t *aBlockingTransactionCount); \
  nsresult AddBlockingTransaction(void); \
  nsresult RemoveBlockingTransaction(uint32_t *_retval); \
  mozilla::net::SpdyPushCache * GetSpdyPushCache(); \
  void SetSpdyPushCache(mozilla::net::SpdyPushCache * aSpdyPushCache); \
  nsresult AddNonTailRequest(void); \
  nsresult RemoveNonTailRequest(void); \
  [[nodiscard]] nsresult IsContextTailBlocked(nsIRequestTailUnblockCallback *callback, bool *_retval); \
  nsresult CancelTailedRequest(nsIRequestTailUnblockCallback *request); \
  nsresult CancelTailPendingRequests(nsresult aResult); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREQUESTCONTEXT(_to) \
  virtual uint64_t GetID() override { return _to GetID(); } \
  NS_IMETHOD BeginLoad(void) override { return _to BeginLoad(); } \
  NS_IMETHOD DOMContentLoaded(void) override { return _to DOMContentLoaded(); } \
  NS_IMETHOD GetBlockingTransactionCount(uint32_t *aBlockingTransactionCount) override { return _to GetBlockingTransactionCount(aBlockingTransactionCount); } \
  NS_IMETHOD AddBlockingTransaction(void) override { return _to AddBlockingTransaction(); } \
  NS_IMETHOD RemoveBlockingTransaction(uint32_t *_retval) override { return _to RemoveBlockingTransaction(_retval); } \
  virtual mozilla::net::SpdyPushCache * GetSpdyPushCache() override { return _to GetSpdyPushCache(); } \
  virtual void SetSpdyPushCache(mozilla::net::SpdyPushCache * aSpdyPushCache) override { return _to SetSpdyPushCache(aSpdyPushCache); } \
  NS_IMETHOD AddNonTailRequest(void) override { return _to AddNonTailRequest(); } \
  NS_IMETHOD RemoveNonTailRequest(void) override { return _to RemoveNonTailRequest(); } \
  [[nodiscard]] NS_IMETHOD IsContextTailBlocked(nsIRequestTailUnblockCallback *callback, bool *_retval) override { return _to IsContextTailBlocked(callback, _retval); } \
  NS_IMETHOD CancelTailedRequest(nsIRequestTailUnblockCallback *request) override { return _to CancelTailedRequest(request); } \
  NS_IMETHOD CancelTailPendingRequests(nsresult aResult) override { return _to CancelTailPendingRequests(aResult); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREQUESTCONTEXT(_to) \
  virtual uint64_t GetID() override; \
  NS_IMETHOD BeginLoad(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeginLoad(); } \
  NS_IMETHOD DOMContentLoaded(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DOMContentLoaded(); } \
  NS_IMETHOD GetBlockingTransactionCount(uint32_t *aBlockingTransactionCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlockingTransactionCount(aBlockingTransactionCount); } \
  NS_IMETHOD AddBlockingTransaction(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddBlockingTransaction(); } \
  NS_IMETHOD RemoveBlockingTransaction(uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveBlockingTransaction(_retval); } \
  virtual mozilla::net::SpdyPushCache * GetSpdyPushCache() override; \
  virtual void SetSpdyPushCache(mozilla::net::SpdyPushCache * aSpdyPushCache) override; \
  NS_IMETHOD AddNonTailRequest(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddNonTailRequest(); } \
  NS_IMETHOD RemoveNonTailRequest(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveNonTailRequest(); } \
  [[nodiscard]] NS_IMETHOD IsContextTailBlocked(nsIRequestTailUnblockCallback *callback, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsContextTailBlocked(callback, _retval); } \
  NS_IMETHOD CancelTailedRequest(nsIRequestTailUnblockCallback *request) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelTailedRequest(request); } \
  NS_IMETHOD CancelTailPendingRequests(nsresult aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelTailPendingRequests(aResult); } 


/* starting interface:    nsIRequestContextService */
#define NS_IREQUESTCONTEXTSERVICE_IID_STR "7fcbf4da-d828-4acc-b144-e5435198f727"

#define NS_IREQUESTCONTEXTSERVICE_IID \
  {0x7fcbf4da, 0xd828, 0x4acc, \
    { 0xb1, 0x44, 0xe5, 0x43, 0x51, 0x98, 0xf7, 0x27 }}

class NS_NO_VTABLE nsIRequestContextService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREQUESTCONTEXTSERVICE_IID)

  /* nsIRequestContext getRequestContext (in unsigned long long id); */
  NS_IMETHOD GetRequestContext(uint64_t id, nsIRequestContext **_retval) = 0;

  /* nsIRequestContext getRequestContextFromLoadGroup (in nsILoadGroup lg); */
  NS_IMETHOD GetRequestContextFromLoadGroup(nsILoadGroup *lg, nsIRequestContext **_retval) = 0;

  /* nsIRequestContext newRequestContext (); */
  NS_IMETHOD NewRequestContext(nsIRequestContext **_retval) = 0;

  /* void removeRequestContext (in unsigned long long id); */
  NS_IMETHOD RemoveRequestContext(uint64_t id) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRequestContextService, NS_IREQUESTCONTEXTSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREQUESTCONTEXTSERVICE \
  NS_IMETHOD GetRequestContext(uint64_t id, nsIRequestContext **_retval) override; \
  NS_IMETHOD GetRequestContextFromLoadGroup(nsILoadGroup *lg, nsIRequestContext **_retval) override; \
  NS_IMETHOD NewRequestContext(nsIRequestContext **_retval) override; \
  NS_IMETHOD RemoveRequestContext(uint64_t id) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREQUESTCONTEXTSERVICE \
  nsresult GetRequestContext(uint64_t id, nsIRequestContext **_retval); \
  nsresult GetRequestContextFromLoadGroup(nsILoadGroup *lg, nsIRequestContext **_retval); \
  nsresult NewRequestContext(nsIRequestContext **_retval); \
  nsresult RemoveRequestContext(uint64_t id); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREQUESTCONTEXTSERVICE(_to) \
  NS_IMETHOD GetRequestContext(uint64_t id, nsIRequestContext **_retval) override { return _to GetRequestContext(id, _retval); } \
  NS_IMETHOD GetRequestContextFromLoadGroup(nsILoadGroup *lg, nsIRequestContext **_retval) override { return _to GetRequestContextFromLoadGroup(lg, _retval); } \
  NS_IMETHOD NewRequestContext(nsIRequestContext **_retval) override { return _to NewRequestContext(_retval); } \
  NS_IMETHOD RemoveRequestContext(uint64_t id) override { return _to RemoveRequestContext(id); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREQUESTCONTEXTSERVICE(_to) \
  NS_IMETHOD GetRequestContext(uint64_t id, nsIRequestContext **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestContext(id, _retval); } \
  NS_IMETHOD GetRequestContextFromLoadGroup(nsILoadGroup *lg, nsIRequestContext **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestContextFromLoadGroup(lg, _retval); } \
  NS_IMETHOD NewRequestContext(nsIRequestContext **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewRequestContext(_retval); } \
  NS_IMETHOD RemoveRequestContext(uint64_t id) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveRequestContext(id); } 


#endif /* __gen_nsIRequestContext_h__ */
