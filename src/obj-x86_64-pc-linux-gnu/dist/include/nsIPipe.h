/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIPipe.idl
 */

#ifndef __gen_nsIPipe_h__
#define __gen_nsIPipe_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAsyncInputStream; /* forward declaration */

class nsIAsyncOutputStream; /* forward declaration */


/* starting interface:    nsIPipe */
#define NS_IPIPE_IID_STR "25d0de93-685e-4ea4-95d3-d884e31df63c"

#define NS_IPIPE_IID \
  {0x25d0de93, 0x685e, 0x4ea4, \
    { 0x95, 0xd3, 0xd8, 0x84, 0xe3, 0x1d, 0xf6, 0x3c }}

class NS_NO_VTABLE nsIPipe : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPIPE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPipe;

  /* [must_use] void init (in boolean nonBlockingInput, in boolean nonBlockingOutput, in unsigned long segmentSize, in unsigned long segmentCount); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Init(bool nonBlockingInput, bool nonBlockingOutput, uint32_t segmentSize, uint32_t segmentCount) = 0;

  /* [must_use] readonly attribute nsIAsyncInputStream inputStream; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetInputStream(nsIAsyncInputStream **aInputStream) = 0;

  /* [must_use] readonly attribute nsIAsyncOutputStream outputStream; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetOutputStream(nsIAsyncOutputStream **aOutputStream) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPipe, NS_IPIPE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPIPE \
  [[nodiscard]] NS_IMETHOD Init(bool nonBlockingInput, bool nonBlockingOutput, uint32_t segmentSize, uint32_t segmentCount) override; \
  [[nodiscard]] NS_IMETHOD GetInputStream(nsIAsyncInputStream **aInputStream) override; \
  [[nodiscard]] NS_IMETHOD GetOutputStream(nsIAsyncOutputStream **aOutputStream) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPIPE \
  [[nodiscard]] nsresult Init(bool nonBlockingInput, bool nonBlockingOutput, uint32_t segmentSize, uint32_t segmentCount); \
  [[nodiscard]] nsresult GetInputStream(nsIAsyncInputStream **aInputStream); \
  [[nodiscard]] nsresult GetOutputStream(nsIAsyncOutputStream **aOutputStream); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPIPE(_to) \
  [[nodiscard]] NS_IMETHOD Init(bool nonBlockingInput, bool nonBlockingOutput, uint32_t segmentSize, uint32_t segmentCount) override { return _to Init(nonBlockingInput, nonBlockingOutput, segmentSize, segmentCount); } \
  [[nodiscard]] NS_IMETHOD GetInputStream(nsIAsyncInputStream **aInputStream) override { return _to GetInputStream(aInputStream); } \
  [[nodiscard]] NS_IMETHOD GetOutputStream(nsIAsyncOutputStream **aOutputStream) override { return _to GetOutputStream(aOutputStream); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPIPE(_to) \
  [[nodiscard]] NS_IMETHOD Init(bool nonBlockingInput, bool nonBlockingOutput, uint32_t segmentSize, uint32_t segmentCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(nonBlockingInput, nonBlockingOutput, segmentSize, segmentCount); } \
  [[nodiscard]] NS_IMETHOD GetInputStream(nsIAsyncInputStream **aInputStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInputStream(aInputStream); } \
  [[nodiscard]] NS_IMETHOD GetOutputStream(nsIAsyncOutputStream **aOutputStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOutputStream(aOutputStream); } 


/* starting interface:    nsISearchableInputStream */
#define NS_ISEARCHABLEINPUTSTREAM_IID_STR "8c39ef62-f7c9-11d4-98f5-001083010e9b"

#define NS_ISEARCHABLEINPUTSTREAM_IID \
  {0x8c39ef62, 0xf7c9, 0x11d4, \
    { 0x98, 0xf5, 0x00, 0x10, 0x83, 0x01, 0x0e, 0x9b }}

class NS_NO_VTABLE nsISearchableInputStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISEARCHABLEINPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISearchableInputStream;

  /* void search (in string forString, in boolean ignoreCase, out boolean found, out unsigned long offsetSearchedTo); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Search(const char * forString, bool ignoreCase, bool *found, uint32_t *offsetSearchedTo) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISearchableInputStream, NS_ISEARCHABLEINPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISEARCHABLEINPUTSTREAM \
  NS_IMETHOD Search(const char * forString, bool ignoreCase, bool *found, uint32_t *offsetSearchedTo) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISEARCHABLEINPUTSTREAM \
  nsresult Search(const char * forString, bool ignoreCase, bool *found, uint32_t *offsetSearchedTo); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISEARCHABLEINPUTSTREAM(_to) \
  NS_IMETHOD Search(const char * forString, bool ignoreCase, bool *found, uint32_t *offsetSearchedTo) override { return _to Search(forString, ignoreCase, found, offsetSearchedTo); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISEARCHABLEINPUTSTREAM(_to) \
  NS_IMETHOD Search(const char * forString, bool ignoreCase, bool *found, uint32_t *offsetSearchedTo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Search(forString, ignoreCase, found, offsetSearchedTo); } 


class nsIInputStream;
class nsIOutputStream;
/**
 * NS_NewPipe2
 *
 * This function supersedes NS_NewPipe.  It differs from NS_NewPipe in two
 * major ways:
 *  (1) returns nsIAsyncInputStream and nsIAsyncOutputStream, so it is
 *      not necessary to QI in order to access these interfaces.
 *  (2) the size of the pipe is determined by the number of segments
 *      times the size of each segment.
 *
 * @param pipeIn
 *        resulting input end of the pipe
 * @param pipeOut
 *        resulting output end of the pipe
 * @param nonBlockingInput
 *        true specifies non-blocking input stream behavior
 * @param nonBlockingOutput
 *        true specifies non-blocking output stream behavior
 * @param segmentSize
 *        specifies the segment size in bytes (pass 0 to use default value)
 * @param segmentCount
 *        specifies the max number of segments (pass 0 to use default value)
 *        passing UINT32_MAX here causes the pipe to have "infinite" space.
 *        this mode can be useful in some cases, but should always be used with
 *        caution.  the default value for this parameter is a finite value.
 */
[[nodiscard]] extern nsresult
NS_NewPipe2(nsIAsyncInputStream **pipeIn,
            nsIAsyncOutputStream **pipeOut,
            bool nonBlockingInput = false,
            bool nonBlockingOutput = false,
            uint32_t segmentSize = 0,
            uint32_t segmentCount = 0);
/**
 * NS_NewPipe
 *
 * Preserved for backwards compatibility.  Plus, this interface is more
 * amiable in certain contexts (e.g., when you don't need the pipe's async
 * capabilities).
 *
 * @param pipeIn
 *        resulting input end of the pipe
 * @param pipeOut
 *        resulting output end of the pipe
 * @param segmentSize
 *        specifies the segment size in bytes (pass 0 to use default value)
 * @param maxSize
 *        specifies the max size of the pipe (pass 0 to use default value)
 *        number of segments is maxSize / segmentSize, and maxSize must be a
 *        multiple of segmentSize.  passing UINT32_MAX here causes the
 *        pipe to have "infinite" space.  this mode can be useful in some
 *        cases, but should always be used with caution.  the default value
 *        for this parameter is a finite value.
 * @param nonBlockingInput
 *        true specifies non-blocking input stream behavior
 * @param nonBlockingOutput
 *        true specifies non-blocking output stream behavior
 */
[[nodiscard]] extern nsresult
NS_NewPipe(nsIInputStream **pipeIn,
           nsIOutputStream **pipeOut,
           uint32_t segmentSize = 0,
           uint32_t maxSize = 0,
           bool nonBlockingInput = false,
           bool nonBlockingOutput = false);

#endif /* __gen_nsIPipe_h__ */
