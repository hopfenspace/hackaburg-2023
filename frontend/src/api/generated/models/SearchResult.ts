/* tslint:disable */
/* eslint-disable */
/**
 * backend
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface SearchResult
 */
export interface SearchResult {
    /**
     * 
     * @type {string}
     * @memberof SearchResult
     */
    uuid: string;
    /**
     * 
     * @type {string}
     * @memberof SearchResult
     */
    name: string;
    /**
     * 
     * @type {string}
     * @memberof SearchResult
     */
    quantity?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SearchResult
     */
    description?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SearchResult
     */
    image?: string | null;
    /**
     * 
     * @type {string}
     * @memberof SearchResult
     */
    mainCategory: string;
}

/**
 * Check if a given object implements the SearchResult interface.
 */
export function instanceOfSearchResult(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "uuid" in value;
    isInstance = isInstance && "name" in value;
    isInstance = isInstance && "mainCategory" in value;

    return isInstance;
}

export function SearchResultFromJSON(json: any): SearchResult {
    return SearchResultFromJSONTyped(json, false);
}

export function SearchResultFromJSONTyped(json: any, ignoreDiscriminator: boolean): SearchResult {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'uuid': json['uuid'],
        'name': json['name'],
        'quantity': !exists(json, 'quantity') ? undefined : json['quantity'],
        'description': !exists(json, 'description') ? undefined : json['description'],
        'image': !exists(json, 'image') ? undefined : json['image'],
        'mainCategory': json['main_category'],
    };
}

export function SearchResultToJSON(value?: SearchResult | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'uuid': value.uuid,
        'name': value.name,
        'quantity': value.quantity,
        'description': value.description,
        'image': value.image,
        'main_category': value.mainCategory,
    };
}

