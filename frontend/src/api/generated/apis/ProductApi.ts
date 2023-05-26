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


import * as runtime from '../runtime';
import type {
  ApiErrorResponse,
  CreateProductRequest,
  ImageState,
  ProductSchema,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    CreateProductRequestFromJSON,
    CreateProductRequestToJSON,
    ImageStateFromJSON,
    ImageStateToJSON,
    ProductSchemaFromJSON,
    ProductSchemaToJSON,
} from '../models';

export interface CreateProductOperationRequest {
    createProductRequest: CreateProductRequest;
}

export interface GetProductRequest {
    uuid: string;
}

export interface GetProductImagesRequest {
    uuid: string;
}

/**
 * 
 */
export class ProductApi extends runtime.BaseAPI {

    /**
     */
    async createProductRaw(requestParameters: CreateProductOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ProductSchema>> {
        if (requestParameters.createProductRequest === null || requestParameters.createProductRequest === undefined) {
            throw new runtime.RequiredError('createProductRequest','Required parameter requestParameters.createProductRequest was null or undefined when calling createProduct.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/product`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreateProductRequestToJSON(requestParameters.createProductRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ProductSchemaFromJSON(jsonValue));
    }

    /**
     */
    async createProduct(requestParameters: CreateProductOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ProductSchema> {
        const response = await this.createProductRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async getProductRaw(requestParameters: GetProductRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ProductSchema>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getProduct.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/product/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ProductSchemaFromJSON(jsonValue));
    }

    /**
     */
    async getProduct(requestParameters: GetProductRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ProductSchema> {
        const response = await this.getProductRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     */
    async getProductImagesRaw(requestParameters: GetProductImagesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ImageState>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getProductImages.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/product/{uuid}/image`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ImageStateFromJSON(jsonValue));
    }

    /**
     */
    async getProductImages(requestParameters: GetProductImagesRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ImageState> {
        const response = await this.getProductImagesRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
