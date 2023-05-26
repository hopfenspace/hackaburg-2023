import { handleError } from "./error";
import { Configuration, SearchApi } from "./generated";

/** Database id i.e. and u32 */
export type ID = number;

/** Hyphen separated uuid */
export type UUID = string;

const configuration = new Configuration({
    basePath: window.location.origin,
});
const searchApi = new SearchApi(configuration);

export const Api = {
    search: (q: string) => handleError(searchApi.postSearch({ q })),
    image: (uuid: UUID) => handleError(searchApi.getProductImages({ uuid })),
};
