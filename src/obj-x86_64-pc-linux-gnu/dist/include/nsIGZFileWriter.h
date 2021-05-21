/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIGZFileWriter.idl
 */

#ifndef __gen_nsIGZFileWriter_h__
#define __gen_nsIGZFileWriter_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsDependentString.h"
#include <stdio.h>
class nsIFile; /* forward declaration */


/* starting interface:    nsIGZFileWriter */
#define NS_IGZFILEWRITER_IID_STR "6bd5642c-1b90-4499-ba4b-199f27efaba5"

#define NS_IGZFILEWRITER_IID \
  {0x6bd5642c, 0x1b90, 0x4499, \
    { 0xba, 0x4b, 0x19, 0x9f, 0x27, 0xef, 0xab, 0xa5 }}

class nsIGZFileWriter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IGZFILEWRITER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIGZFileWriter;

  /* [must_use] void init (in nsIFile file); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Init(nsIFile *file) = 0;

  /* [must_use,noscript] void initANSIFileDesc (in FILE file); */
  [[nodiscard]] NS_IMETHOD InitANSIFileDesc(FILE * file) = 0;

  /* [must_use] void write (in AUTF8String str); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Write(const nsACString& str) = 0;

   /**
   * Write the given char* to the file (not including the null-terminator).
   */
  [[nodiscard]] nsresult Write(const char* str)
  {
    return Write(str, strlen(str));
  }
  /**
   * Write |length| bytes of |str| to the file.
   */
  [[nodiscard]] nsresult Write(const char* str, uint32_t len)
  {
    return Write(nsDependentCSubstring(str, len));
  }
    /* void finish (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Finish(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIGZFileWriter, NS_IGZFILEWRITER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIGZFILEWRITER \
  [[nodiscard]] NS_IMETHOD Init(nsIFile *file) override; \
  [[nodiscard]] NS_IMETHOD InitANSIFileDesc(FILE * file) override; \
  [[nodiscard]] NS_IMETHOD Write(const nsACString& str) override; \
  NS_IMETHOD Finish(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIGZFILEWRITER \
  [[nodiscard]] nsresult Init(nsIFile *file); \
  [[nodiscard]] nsresult InitANSIFileDesc(FILE * file); \
  [[nodiscard]] nsresult Write(const nsACString& str); \
  nsresult Finish(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIGZFILEWRITER(_to) \
  [[nodiscard]] NS_IMETHOD Init(nsIFile *file) override { return _to Init(file); } \
  [[nodiscard]] NS_IMETHOD InitANSIFileDesc(FILE * file) override { return _to InitANSIFileDesc(file); } \
  [[nodiscard]] NS_IMETHOD Write(const nsACString& str) override { return _to Write(str); } \
  NS_IMETHOD Finish(void) override { return _to Finish(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIGZFILEWRITER(_to) \
  [[nodiscard]] NS_IMETHOD Init(nsIFile *file) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(file); } \
  [[nodiscard]] NS_IMETHOD InitANSIFileDesc(FILE * file) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitANSIFileDesc(file); } \
  [[nodiscard]] NS_IMETHOD Write(const nsACString& str) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Write(str); } \
  NS_IMETHOD Finish(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Finish(); } 


#endif /* __gen_nsIGZFileWriter_h__ */
