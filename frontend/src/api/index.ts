import { handleError } from "./error";
import { Configuration, SearchApi } from "./generated";

const configuration = new Configuration({
    basePath: window.location.origin,
});
const searchApi = new SearchApi(configuration);

export const Api = {
    search: (query: string) => handleError(searchApi.postSearch({ searchInput: { q: query } })),
};
