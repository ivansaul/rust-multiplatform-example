//
//  TaskCreateView.swift
//  Rusty
//
//  Created by ivansaul on 9/6/26.
//

import Factory
import RustyCore
import SwiftUI

struct CreateTaskView: View {
    @Environment(\.dismiss) var dismiss
    @Injected(\.taskCreateViewModel) private var viewModel

    var body: some View {
        @Bindable var viewModel = viewModel
        VStack {
            TextField("New task...", text: $viewModel.title)
                .padding()
                .overlay {
                    RoundedRectangle(cornerRadius: 15)
                        .stroke(Color.gray, lineWidth: 2)
                }

            Button("Add Task") {
                Task {
                    await viewModel.create_task()
                    dismiss()
                }
            }
            .buttonStyle(.borderedProminent)
        }
        .padding()
    }
}

#Preview {
    CreateTaskView()
}
