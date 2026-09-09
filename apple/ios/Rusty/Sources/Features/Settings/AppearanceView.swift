//
//  AppearanceView.swift
//  Rusty
//
//  Created by ivansaul on 8/14/26.
//

import Factory
import RustyCore
import SwiftUI

struct AppearanceView: View {
    @Injected(\.settingsViewModel) private var settingsViewModel

    var body: some View {
        @Bindable var settingsViewModel = settingsViewModel

        List {
            Picker("Theme", selection: $settingsViewModel.selectedColorScheme) {
                Text("Light").tag(ColorSchemeSelection.light)
                Text("Dark").tag(ColorSchemeSelection.dark)
                Text("System").tag(ColorSchemeSelection.system)
            }
            .pickerStyle(.inline)
        }
        .navigationTitle("Appearance")
    }
}

#Preview {
    AppearanceView()
}
