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
import type { ImageState } from './ImageState';
import {
    ImageStateFromJSON,
    ImageStateFromJSONTyped,
    ImageStateToJSON,
} from './ImageState';

/**
 * 
 * @export
 * @interface ImageResult
 */
export interface ImageResult {
    /**
     * 
     * @type {ImageState}
     * @memberof ImageResult
     */
    imageState: ImageState;
    /**
     * 
     * @type {string}
     * @memberof ImageResult
     */
    image?: string | null;
}

/**
 * Check if a given object implements the ImageResult interface.
 */
export function instanceOfImageResult(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "imageState" in value;

    return isInstance;
}

export function ImageResultFromJSON(json: any): ImageResult {
    return ImageResultFromJSONTyped(json, false);
}

export function ImageResultFromJSONTyped(json: any, ignoreDiscriminator: boolean): ImageResult {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'imageState': ImageStateFromJSON(json['image_state']),
        'image': !exists(json, 'image') ? undefined : json['image'],
    };
}

export function ImageResultToJSON(value?: ImageResult | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'image_state': ImageStateToJSON(value.imageState),
        'image': value.image,
    };
}
