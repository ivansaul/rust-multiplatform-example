//
//  AppearanceView.swift
//  Rusty
//
//  Created by ivansaul on 8/14/26.
//

import Factory
import SwiftUI

struct AppearanceView: View {
    @Injected(\.appearanceViewModel) private var appearanceViewModel

    var body: some View {
        @Bindable var appearance = appearanceViewModel

        List {
            Picker("Theme", selection: $appearance.theme) {
                ForEach(ThemePreference.allCases, id: \.self) { theme in
                    Label(
                        theme.title,
                        systemImage: theme.icon
                    )
                    .tag(theme)
                }
            }
            .pickerStyle(.inline)
        }
        .navigationTitle("Apariencia")
    }
}

#Preview {
    AppearanceView()
}
