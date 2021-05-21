/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/simpledb/nsISDBConnection.idl
 */

#ifndef __gen_nsISDBConnection_h__
#define __gen_nsISDBConnection_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class nsISDBCloseCallback; /* forward declaration */

class nsISDBRequest; /* forward declaration */


/* starting interface:    nsISDBConnection */
#define NS_ISDBCONNECTION_IID_STR "ea420fdd-548f-44f9-9286-59aad6a40f01"

#define NS_ISDBCONNECTION_IID \
  {0xea420fdd, 0x548f, 0x44f9, \
    { 0x92, 0x86, 0x59, 0xaa, 0xd6, 0xa4, 0x0f, 0x01 }}

class NS_NO_VTABLE nsISDBConnection : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISDBCONNECTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISDBConnection;

  /* [must_use] void init (in nsIPrincipal aPrincipal, [optional] in ACString aPersistenceType); */
  [[nodiscard]] NS_IMETHOD Init(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType) = 0;

  /* [must_use] nsISDBRequest open (in AString aName); */
  [[nodiscard]] NS_IMETHOD Open(const nsAString& aName, nsISDBRequest **_retval) = 0;

  /* [must_use] nsISDBRequest seek (in unsigned long long offset); */
  [[nodiscard]] NS_IMETHOD Seek(uint64_t offset, nsISDBRequest **_retval) = 0;

  /* [must_use] nsISDBRequest read (in unsigned long long size); */
  [[nodiscard]] NS_IMETHOD Read(uint64_t size, nsISDBRequest **_retval) = 0;

  /* [implicit_jscontext,must_use] nsISDBRequest write (in jsval value); */
  [[nodiscard]] NS_IMETHOD Write(JS::HandleValue value, JSContext* cx, nsISDBRequest **_retval) = 0;

  /* [must_use] nsISDBRequest close (); */
  [[nodiscard]] NS_IMETHOD Close(nsISDBRequest **_retval) = 0;

  /* attribute nsISDBCloseCallback closeCallback; */
  NS_IMETHOD GetCloseCallback(nsISDBCloseCallback **aCloseCallback) = 0;
  NS_IMETHOD SetCloseCallback(nsISDBCloseCallback *aCloseCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISDBConnection, NS_ISDBCONNECTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISDBCONNECTION \
  [[nodiscard]] NS_IMETHOD Init(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType) override; \
  [[nodiscard]] NS_IMETHOD Open(const nsAString& aName, nsISDBRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD Seek(uint64_t offset, nsISDBRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD Read(uint64_t size, nsISDBRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD Write(JS::HandleValue value, JSContext* cx, nsISDBRequest **_retval) override; \
  [[nodiscard]] NS_IMETHOD Close(nsISDBRequest **_retval) override; \
  NS_IMETHOD GetCloseCallback(nsISDBCloseCallback **aCloseCallback) override; \
  NS_IMETHOD SetCloseCallback(nsISDBCloseCallback *aCloseCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISDBCONNECTION \
  [[nodiscard]] nsresult Init(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType); \
  [[nodiscard]] nsresult Open(const nsAString& aName, nsISDBRequest **_retval); \
  [[nodiscard]] nsresult Seek(uint64_t offset, nsISDBRequest **_retval); \
  [[nodiscard]] nsresult Read(uint64_t size, nsISDBRequest **_retval); \
  [[nodiscard]] nsresult Write(JS::HandleValue value, JSContext* cx, nsISDBRequest **_retval); \
  [[nodiscard]] nsresult Close(nsISDBRequest **_retval); \
  nsresult GetCloseCallback(nsISDBCloseCallback **aCloseCallback); \
  nsresult SetCloseCallback(nsISDBCloseCallback *aCloseCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISDBCONNECTION(_to) \
  [[nodiscard]] NS_IMETHOD Init(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType) override { return _to Init(aPrincipal, aPersistenceType); } \
  [[nodiscard]] NS_IMETHOD Open(const nsAString& aName, nsISDBRequest **_retval) override { return _to Open(aName, _retval); } \
  [[nodiscard]] NS_IMETHOD Seek(uint64_t offset, nsISDBRequest **_retval) override { return _to Seek(offset, _retval); } \
  [[nodiscard]] NS_IMETHOD Read(uint64_t size, nsISDBRequest **_retval) override { return _to Read(size, _retval); } \
  [[nodiscard]] NS_IMETHOD Write(JS::HandleValue value, JSContext* cx, nsISDBRequest **_retval) override { return _to Write(value, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD Close(nsISDBRequest **_retval) override { return _to Close(_retval); } \
  NS_IMETHOD GetCloseCallback(nsISDBCloseCallback **aCloseCallback) override { return _to GetCloseCallback(aCloseCallback); } \
  NS_IMETHOD SetCloseCallback(nsISDBCloseCallback *aCloseCallback) override { return _to SetCloseCallback(aCloseCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISDBCONNECTION(_to) \
  [[nodiscard]] NS_IMETHOD Init(nsIPrincipal *aPrincipal, const nsACString& aPersistenceType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aPrincipal, aPersistenceType); } \
  [[nodiscard]] NS_IMETHOD Open(const nsAString& aName, nsISDBRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Open(aName, _retval); } \
  [[nodiscard]] NS_IMETHOD Seek(uint64_t offset, nsISDBRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Seek(offset, _retval); } \
  [[nodiscard]] NS_IMETHOD Read(uint64_t size, nsISDBRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Read(size, _retval); } \
  [[nodiscard]] NS_IMETHOD Write(JS::HandleValue value, JSContext* cx, nsISDBRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Write(value, cx, _retval); } \
  [[nodiscard]] NS_IMETHOD Close(nsISDBRequest **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Close(_retval); } \
  NS_IMETHOD GetCloseCallback(nsISDBCloseCallback **aCloseCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCloseCallback(aCloseCallback); } \
  NS_IMETHOD SetCloseCallback(nsISDBCloseCallback *aCloseCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCloseCallback(aCloseCallback); } 


#endif /* __gen_nsISDBConnection_h__ */
