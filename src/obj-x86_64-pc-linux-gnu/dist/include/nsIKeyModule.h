/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIKeyModule.idl
 */

#ifndef __gen_nsIKeyModule_h__
#define __gen_nsIKeyModule_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
 /* forward declaration */
 typedef struct PK11SymKeyStr PK11SymKey;

/* starting interface:    nsIKeyObject */
#define NS_IKEYOBJECT_IID_STR "ee2dc516-ba7b-4e77-89fe-c43b886ef715"

#define NS_IKEYOBJECT_IID \
  {0xee2dc516, 0xba7b, 0x4e77, \
    { 0x89, 0xfe, 0xc4, 0x3b, 0x88, 0x6e, 0xf7, 0x15 }}

class NS_NO_VTABLE nsIKeyObject : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYOBJECT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyObject;

  enum {
    SYM_KEY = 1,
    HMAC = 257
  };

  /* [must_use,noscript] void initKey (in short aAlgorithm, in PK11SymKeyPtr aKey); */
  [[nodiscard]] NS_IMETHOD InitKey(int16_t aAlgorithm, PK11SymKey * aKey) = 0;

  /* [must_use,noscript] PK11SymKeyPtr getKeyObj (); */
  [[nodiscard]] NS_IMETHOD GetKeyObj(PK11SymKey * * _retval) = 0;

  /* [must_use] short getType (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetType(int16_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyObject, NS_IKEYOBJECT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYOBJECT \
  [[nodiscard]] NS_IMETHOD InitKey(int16_t aAlgorithm, PK11SymKey * aKey) override; \
  [[nodiscard]] NS_IMETHOD GetKeyObj(PK11SymKey * * _retval) override; \
  [[nodiscard]] NS_IMETHOD GetType(int16_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYOBJECT \
  [[nodiscard]] nsresult InitKey(int16_t aAlgorithm, PK11SymKey * aKey); \
  [[nodiscard]] nsresult GetKeyObj(PK11SymKey * * _retval); \
  [[nodiscard]] nsresult GetType(int16_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYOBJECT(_to) \
  [[nodiscard]] NS_IMETHOD InitKey(int16_t aAlgorithm, PK11SymKey * aKey) override { return _to InitKey(aAlgorithm, aKey); } \
  [[nodiscard]] NS_IMETHOD GetKeyObj(PK11SymKey * * _retval) override { return _to GetKeyObj(_retval); } \
  [[nodiscard]] NS_IMETHOD GetType(int16_t *_retval) override { return _to GetType(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYOBJECT(_to) \
  [[nodiscard]] NS_IMETHOD InitKey(int16_t aAlgorithm, PK11SymKey * aKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitKey(aAlgorithm, aKey); } \
  [[nodiscard]] NS_IMETHOD GetKeyObj(PK11SymKey * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeyObj(_retval); } \
  [[nodiscard]] NS_IMETHOD GetType(int16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(_retval); } 


/* starting interface:    nsIKeyObjectFactory */
#define NS_IKEYOBJECTFACTORY_IID_STR "838bdbf1-8244-448f-8bcd-cede70227d75"

#define NS_IKEYOBJECTFACTORY_IID \
  {0x838bdbf1, 0x8244, 0x448f, \
    { 0x8b, 0xcd, 0xce, 0xde, 0x70, 0x22, 0x7d, 0x75 }}

class NS_NO_VTABLE nsIKeyObjectFactory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IKEYOBJECTFACTORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIKeyObjectFactory;

  /* [must_use] nsIKeyObject keyFromString (in short aAlgorithm, in ACString aKey); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD KeyFromString(int16_t aAlgorithm, const nsACString& aKey, nsIKeyObject **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIKeyObjectFactory, NS_IKEYOBJECTFACTORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIKEYOBJECTFACTORY \
  [[nodiscard]] NS_IMETHOD KeyFromString(int16_t aAlgorithm, const nsACString& aKey, nsIKeyObject **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIKEYOBJECTFACTORY \
  [[nodiscard]] nsresult KeyFromString(int16_t aAlgorithm, const nsACString& aKey, nsIKeyObject **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIKEYOBJECTFACTORY(_to) \
  [[nodiscard]] NS_IMETHOD KeyFromString(int16_t aAlgorithm, const nsACString& aKey, nsIKeyObject **_retval) override { return _to KeyFromString(aAlgorithm, aKey, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIKEYOBJECTFACTORY(_to) \
  [[nodiscard]] NS_IMETHOD KeyFromString(int16_t aAlgorithm, const nsACString& aKey, nsIKeyObject **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->KeyFromString(aAlgorithm, aKey, _retval); } 


#endif /* __gen_nsIKeyModule_h__ */
