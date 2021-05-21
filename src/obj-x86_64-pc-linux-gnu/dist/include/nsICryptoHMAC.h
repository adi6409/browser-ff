/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICryptoHMAC.idl
 */

#ifndef __gen_nsICryptoHMAC_h__
#define __gen_nsICryptoHMAC_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */

class nsIKeyObject; /* forward declaration */


/* starting interface:    nsICryptoHMAC */
#define NS_ICRYPTOHMAC_IID_STR "8feb4c7c-1641-4a7b-bc6d-1964e2099497"

#define NS_ICRYPTOHMAC_IID \
  {0x8feb4c7c, 0x1641, 0x4a7b, \
    { 0xbc, 0x6d, 0x19, 0x64, 0xe2, 0x09, 0x94, 0x97 }}

class NS_NO_VTABLE nsICryptoHMAC : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICRYPTOHMAC_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICryptoHMAC;

  enum {
    MD5 = 2,
    SHA1 = 3,
    SHA256 = 4,
    SHA384 = 5,
    SHA512 = 6
  };

  /* [must_use] void init (in unsigned long aAlgorithm, in nsIKeyObject aKeyObject); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Init(uint32_t aAlgorithm, nsIKeyObject *aKeyObject) = 0;

  /* [must_use] void update ([array, size_is (aLen), const] in octet aData, in unsigned long aLen); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Update(const uint8_t *aData, uint32_t aLen) = 0;

  /* [must_use] void updateFromStream (in nsIInputStream aStream, in unsigned long aLen); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD UpdateFromStream(nsIInputStream *aStream, uint32_t aLen) = 0;

  /* [must_use] ACString finish (in boolean aASCII); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Finish(bool aASCII, nsACString& _retval) = 0;

  /* [must_use] void reset (); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Reset(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICryptoHMAC, NS_ICRYPTOHMAC_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICRYPTOHMAC \
  [[nodiscard]] NS_IMETHOD Init(uint32_t aAlgorithm, nsIKeyObject *aKeyObject) override; \
  [[nodiscard]] NS_IMETHOD Update(const uint8_t *aData, uint32_t aLen) override; \
  [[nodiscard]] NS_IMETHOD UpdateFromStream(nsIInputStream *aStream, uint32_t aLen) override; \
  [[nodiscard]] NS_IMETHOD Finish(bool aASCII, nsACString& _retval) override; \
  [[nodiscard]] NS_IMETHOD Reset(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICRYPTOHMAC \
  [[nodiscard]] nsresult Init(uint32_t aAlgorithm, nsIKeyObject *aKeyObject); \
  [[nodiscard]] nsresult Update(const uint8_t *aData, uint32_t aLen); \
  [[nodiscard]] nsresult UpdateFromStream(nsIInputStream *aStream, uint32_t aLen); \
  [[nodiscard]] nsresult Finish(bool aASCII, nsACString& _retval); \
  [[nodiscard]] nsresult Reset(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICRYPTOHMAC(_to) \
  [[nodiscard]] NS_IMETHOD Init(uint32_t aAlgorithm, nsIKeyObject *aKeyObject) override { return _to Init(aAlgorithm, aKeyObject); } \
  [[nodiscard]] NS_IMETHOD Update(const uint8_t *aData, uint32_t aLen) override { return _to Update(aData, aLen); } \
  [[nodiscard]] NS_IMETHOD UpdateFromStream(nsIInputStream *aStream, uint32_t aLen) override { return _to UpdateFromStream(aStream, aLen); } \
  [[nodiscard]] NS_IMETHOD Finish(bool aASCII, nsACString& _retval) override { return _to Finish(aASCII, _retval); } \
  [[nodiscard]] NS_IMETHOD Reset(void) override { return _to Reset(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICRYPTOHMAC(_to) \
  [[nodiscard]] NS_IMETHOD Init(uint32_t aAlgorithm, nsIKeyObject *aKeyObject) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aAlgorithm, aKeyObject); } \
  [[nodiscard]] NS_IMETHOD Update(const uint8_t *aData, uint32_t aLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Update(aData, aLen); } \
  [[nodiscard]] NS_IMETHOD UpdateFromStream(nsIInputStream *aStream, uint32_t aLen) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateFromStream(aStream, aLen); } \
  [[nodiscard]] NS_IMETHOD Finish(bool aASCII, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Finish(aASCII, _retval); } \
  [[nodiscard]] NS_IMETHOD Reset(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reset(); } 


#endif /* __gen_nsICryptoHMAC_h__ */
