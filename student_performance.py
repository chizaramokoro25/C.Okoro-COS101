# ====================================================================
# Student Performance Analysis for Computer Science Department Year 1
# ====================================================================
import pandas as pd
import matplotlib.pyplot as plt

data = {
    'Name': ['Afolabi Derek', 'Onyeme Nneoma', 'Okoro Chizaram', 'Olu-Monday Ifedayo', 'Charleston Peter', 'Osundare Victoria'],
    'COS 101': [85, 72, 90, 60, 95, 61],
    'PAU CSC 111': [78, 80, 88, 70, 85, 64],
    'MTH 101': [92, 68, 75, 65, 89, 65],
    'PHY 101': [94, 96, 99, 65, 64, 46],
    'PHY 107': [92, 84, 94, 86, 72, 64],
    'STA 111': [91, 90, 82, 44, 59, 69],
    'PAU CSC 192': [93, 81, 82, 84, 87, 89],
}

df = pd.DataFrame(data)

df['Average'] = df[['COS 101', 'PAU CSC 111', 'MTH 101', 'PHY 101', 'PHY 107', 'STA 111', 'PAU CSC 192']].mean(axis=1)

print("=== Student Scores ===")
print(df)

top_student = df.loc[df['Average'].idxmax()]
bottom_student = df.loc[df['Average'].idxmin()]

print("\n=== Top Student ===")
print(top_student)

print("\n=== Bottom Student ===")
print(bottom_student)

subject_averages = df[['COS 101', 'PAU CSC 111', 'MTH 101', 'PHY 101', 'PHY 107', 'STA 111', 'PAU CSC 192']].mean()
print("\n=== Subject Averages ===")
print(subject_averages)

subject_averages.plot(kind='bar', title='Subject Averages', color=['skyblue', 'lightgreen', 'salmon'])
plt.xlabel('Subjects')
plt.ylabel('Average Score')
plt.show()

df.plot(x='Name', y=['COS 101', 'PAU CSC 111', 'MTH 101', 'PHY 101', 'PHY 107', 'STA 111', 'PAU CSC 192'], kind='line', marker='o', title='Student Performance Over Subjects')
plt.ylabel('Scores')
plt.show()

lowest_subject = subject_averages.idxmin()
print(f"\nSubject that needs the most improvement: {lowest_subject}")
