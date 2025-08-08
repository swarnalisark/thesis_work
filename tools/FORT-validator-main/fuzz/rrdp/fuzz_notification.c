#include <stdio.h>
#include <stdint.h>
#include <stddef.h>

#include "../../src/rrdp.c"        // Use the actual implementation
#include "libxml/parser.h"
#include "types/uri.h"

// Optional: suppress libxml2 error output
__attribute__((constructor))
static void disable_libxml2_errors() {
    xmlSetGenericErrorFunc(NULL, NULL);
}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    const char *tmp_path = "/tmp/fuzz_input.xml";

    FILE *f = fopen(tmp_path, "wb");
    if (!f) return 0;
    fwrite(data, 1, size, f);
    fclose(f);

    struct rpki_uri *uri = uri_create_cache(tmp_path);
    if (!uri) return 0;

    struct update_notification result;
    parse_notification(uri, &result);

    uri_refput(uri);
    return 0;
}
