//
//  AsyncValue.swift
//  Rusty
//
//  Created by ivansaul on 9/5/26.
//

import Foundation

public enum AsyncValue<T> {
    case idle
    case loading
    case data(T)
    case error(Error)
}
