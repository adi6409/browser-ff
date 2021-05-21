/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/ipc/nsIDOMProcessChild.idl
 */

#ifndef __gen_nsIDOMProcessChild_h__
#define __gen_nsIDOMProcessChild_h__


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
class ContentChild;
class JSActorManager;
} // namespace dom
} // namespace mozilla
namespace mozilla {
namespace dom {
class JSProcessActorChild; /* webidl JSProcessActorChild */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMProcessChild */
#define NS_IDOMPROCESSCHILD_IID_STR "b0c6e5f3-02f1-4f11-a0af-336fc231f3bf"

#define NS_IDOMPROCESSCHILD_IID \
  {0xb0c6e5f3, 0x02f1, 0x4f11, \
    { 0xa0, 0xaf, 0x33, 0x6f, 0xc2, 0x31, 0xf3, 0xbf }}

class nsIDOMProcessChild : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMPROCESSCHILD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMProcessChild;

   /**
   * Get the nsIDOMProcessChild singleton for this content process. This will
   * either be an InProcessChild in the parent process, or ContentChild in the
   * child process.
   *
   * Implemented in ContentChild.cpp
   */
  static nsIDOMProcessChild* GetSingleton();
    /* [infallible] readonly attribute unsigned long long childID; */
  NS_IMETHOD GetChildID(uint64_t *aChildID) = 0;
  inline uint64_t  GetChildID()
  {
    uint64_t result;
    mozilla::DebugOnly<nsresult> rv = GetChildID(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [implicit_jscontext] JSProcessActorChild getActor (in ACString name); */
  NS_IMETHOD GetActor(const nsACString& name, JSContext* cx, mozilla::dom::JSProcessActorChild **_retval) = 0;

  /* [infallible] readonly attribute boolean canSend; */
  NS_IMETHOD GetCanSend(bool *aCanSend) = 0;
  inline bool  GetCanSend()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetCanSend(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [nostdcall,notxpcom] ContentChildPtr AsContentChild (); */
  virtual mozilla::dom::ContentChild * AsContentChild(void) = 0;

  /* [nostdcall,notxpcom] JSActorManagerPtr AsJSActorManager (); */
  virtual mozilla::dom::JSActorManager * AsJSActorManager(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMProcessChild, NS_IDOMPROCESSCHILD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMPROCESSCHILD \
  using nsIDOMProcessChild::GetChildID; \
  NS_IMETHOD GetChildID(uint64_t *aChildID) override; \
  NS_IMETHOD GetActor(const nsACString& name, JSContext* cx, mozilla::dom::JSProcessActorChild **_retval) override; \
  using nsIDOMProcessChild::GetCanSend; \
  NS_IMETHOD GetCanSend(bool *aCanSend) override; \
  virtual mozilla::dom::ContentChild * AsContentChild(void) override; \
  virtual mozilla::dom::JSActorManager * AsJSActorManager(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMPROCESSCHILD \
  using nsIDOMProcessChild::GetChildID; \
  nsresult GetChildID(uint64_t *aChildID); \
  nsresult GetActor(const nsACString& name, JSContext* cx, mozilla::dom::JSProcessActorChild **_retval); \
  using nsIDOMProcessChild::GetCanSend; \
  nsresult GetCanSend(bool *aCanSend); \
  mozilla::dom::ContentChild * AsContentChild(void); \
  mozilla::dom::JSActorManager * AsJSActorManager(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMPROCESSCHILD(_to) \
  using nsIDOMProcessChild::GetChildID; \
  NS_IMETHOD GetChildID(uint64_t *aChildID) override { return _to GetChildID(aChildID); } \
  NS_IMETHOD GetActor(const nsACString& name, JSContext* cx, mozilla::dom::JSProcessActorChild **_retval) override { return _to GetActor(name, cx, _retval); } \
  using nsIDOMProcessChild::GetCanSend; \
  NS_IMETHOD GetCanSend(bool *aCanSend) override { return _to GetCanSend(aCanSend); } \
  virtual mozilla::dom::ContentChild * AsContentChild(void) override { return _to AsContentChild(); } \
  virtual mozilla::dom::JSActorManager * AsJSActorManager(void) override { return _to AsJSActorManager(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMPROCESSCHILD(_to) \
  NS_IMETHOD GetChildID(uint64_t *aChildID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildID(aChildID); } \
  NS_IMETHOD GetActor(const nsACString& name, JSContext* cx, mozilla::dom::JSProcessActorChild **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActor(name, cx, _retval); } \
  NS_IMETHOD GetCanSend(bool *aCanSend) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanSend(aCanSend); } \
  virtual mozilla::dom::ContentChild * AsContentChild(void) override; \
  virtual mozilla::dom::JSActorManager * AsJSActorManager(void) override; 


#endif /* __gen_nsIDOMProcessChild_h__ */
