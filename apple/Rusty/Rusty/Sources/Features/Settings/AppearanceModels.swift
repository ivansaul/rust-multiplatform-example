//
//  AppearanceModels.swift
//  Rusty
//
//  Created by ivansaul on 8/14/26.
//

import Foundation
import SwiftUI

enum ThemePreference: String, CaseIterable {
    case system
    case light
    case dark

    var title: String {
        switch self {
        case .system:
            "System"
        case .light:
            "Light"
        case .dark:
            "Dark"
        }
    }

    var icon: String {
        switch self {
        case .system:
            "iphone"
        case .light:
            "sun.max"
        case .dark:
            "moon"
        }
    }

    var colorScheme: ColorScheme? {
        switch self {
        case .system:
            nil
        case .light:
            .light
        case .dark:
            .dark
        }
    }
}
