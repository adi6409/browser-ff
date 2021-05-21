/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/ipc/nsIDOMProcessParent.idl
 */

#ifndef __gen_nsIDOMProcessParent_h__
#define __gen_nsIDOMProcessParent_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class ContentParent;
class JSActorManager;
} // namespace dom
} // namespace mozilla
namespace mozilla {
namespace dom {
class JSProcessActorParent; /* webidl JSProcessActorParent */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMProcessParent */
#define NS_IDOMPROCESSPARENT_IID_STR "81fc08b9-c901-471f-ab0d-876362eba770"

#define NS_IDOMPROCESSPARENT_IID \
  {0x81fc08b9, 0xc901, 0x471f, \
    { 0xab, 0x0d, 0x87, 0x63, 0x62, 0xeb, 0xa7, 0x70 }}

class NS_NO_VTABLE nsIDOMProcessParent : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMPROCESSPARENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMProcessParent;

  /* [infallible] readonly attribute unsigned long long childID; */
  NS_IMETHOD GetChildID(uint64_t *aChildID) = 0;
  inline uint64_t  GetChildID()
  {
    uint64_t result;
    mozilla::DebugOnly<nsresult> rv = GetChildID(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute long osPid; */
  NS_IMETHOD GetOsPid(int32_t *aOsPid) = 0;
  inline int32_t  GetOsPid()
  {
    int32_t result;
    mozilla::DebugOnly<nsresult> rv = GetOsPid(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [implicit_jscontext] JSProcessActorParent getActor (in ACString name); */
  NS_IMETHOD GetActor(const nsACString& name, JSContext* cx, mozilla::dom::JSProcessActorParent **_retval) = 0;

  /* [infallible] readonly attribute boolean canSend; */
  NS_IMETHOD GetCanSend(bool *aCanSend) = 0;
  inline bool  GetCanSend()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetCanSend(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [nostdcall,notxpcom] ContentParentPtr AsContentParent (); */
  virtual mozilla::dom::ContentParent * AsContentParent(void) = 0;

  /* [nostdcall,notxpcom] JSActorManagerPtr AsJSActorManager (); */
  virtual mozilla::dom::JSActorManager * AsJSActorManager(void) = 0;

  /* readonly attribute ACString remoteType; */
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMProcessParent, NS_IDOMPROCESSPARENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMPROCESSPARENT \
  using nsIDOMProcessParent::GetChildID; \
  NS_IMETHOD GetChildID(uint64_t *aChildID) override; \
  using nsIDOMProcessParent::GetOsPid; \
  NS_IMETHOD GetOsPid(int32_t *aOsPid) override; \
  NS_IMETHOD GetActor(const nsACString& name, JSContext* cx, mozilla::dom::JSProcessActorParent **_retval) override; \
  using nsIDOMProcessParent::GetCanSend; \
  NS_IMETHOD GetCanSend(bool *aCanSend) override; \
  virtual mozilla::dom::ContentParent * AsContentParent(void) override; \
  virtual mozilla::dom::JSActorManager * AsJSActorManager(void) override; \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMPROCESSPARENT \
  using nsIDOMProcessParent::GetChildID; \
  nsresult GetChildID(uint64_t *aChildID); \
  using nsIDOMProcessParent::GetOsPid; \
  nsresult GetOsPid(int32_t *aOsPid); \
  nsresult GetActor(const nsACString& name, JSContext* cx, mozilla::dom::JSProcessActorParent **_retval); \
  using nsIDOMProcessParent::GetCanSend; \
  nsresult GetCanSend(bool *aCanSend); \
  mozilla::dom::ContentParent * AsContentParent(void); \
  mozilla::dom::JSActorManager * AsJSActorManager(void); \
  nsresult GetRemoteType(nsACString& aRemoteType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMPROCESSPARENT(_to) \
  using nsIDOMProcessParent::GetChildID; \
  NS_IMETHOD GetChildID(uint64_t *aChildID) override { return _to GetChildID(aChildID); } \
  using nsIDOMProcessParent::GetOsPid; \
  NS_IMETHOD GetOsPid(int32_t *aOsPid) override { return _to GetOsPid(aOsPid); } \
  NS_IMETHOD GetActor(const nsACString& name, JSContext* cx, mozilla::dom::JSProcessActorParent **_retval) override { return _to GetActor(name, cx, _retval); } \
  using nsIDOMProcessParent::GetCanSend; \
  NS_IMETHOD GetCanSend(bool *aCanSend) override { return _to GetCanSend(aCanSend); } \
  virtual mozilla::dom::ContentParent * AsContentParent(void) override { return _to AsContentParent(); } \
  virtual mozilla::dom::JSActorManager * AsJSActorManager(void) override { return _to AsJSActorManager(); } \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override { return _to GetRemoteType(aRemoteType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMPROCESSPARENT(_to) \
  NS_IMETHOD GetChildID(uint64_t *aChildID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildID(aChildID); } \
  NS_IMETHOD GetOsPid(int32_t *aOsPid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOsPid(aOsPid); } \
  NS_IMETHOD GetActor(const nsACString& name, JSContext* cx, mozilla::dom::JSProcessActorParent **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActor(name, cx, _retval); } \
  NS_IMETHOD GetCanSend(bool *aCanSend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanSend(aCanSend); } \
  virtual mozilla::dom::ContentParent * AsContentParent(void) override; \
  virtual mozilla::dom::JSActorManager * AsJSActorManager(void) override; \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemoteType(aRemoteType); } 


#endif /* __gen_nsIDOMProcessParent_h__ */
