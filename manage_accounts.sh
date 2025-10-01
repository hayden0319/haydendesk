#!/bin/bash

# FamilyDesk Account Management Script
# Manages user accounts on the API server at nas.haydenstudio.hk:21114

API_URL="http://nas.haydenstudio.hk:21114"

echo "============================================"
echo "  FamilyDesk Account Management"
echo "============================================"
echo ""
echo "1. Create Account"
echo "2. List All Accounts"
echo "3. Test Login"
echo "4. Health Check"
echo "============================================"
read -p "Choose option (1-4): " option
echo ""

case "$option" in
    1)
        echo "=== Create New Account ==="
        read -p "Admin password: " -s admin_pwd
        echo ""
        read -p "New username: " new_user
        read -p "New password: " -s new_pwd
        echo ""

        echo "Available roles:"
        echo "  - admin: Full access to all devices"
        echo "  - family: Access to specific family devices"
        echo "  - student: Limited access for school use"
        read -p "Role (admin/family/student): " role

        read -p "Can modify settings? (true/false): " can_modify

        echo ""
        echo "Device IDs:"
        echo "  - Enter '*' for all devices"
        echo "  - Or comma-separated list (e.g., GRANDMA_PC,DAD_PC)"
        read -p "Device IDs: " devices

        # Convert comma-separated to JSON array
        if [ "$devices" == "*" ]; then
            device_json='["*"]'
        else
            IFS=',' read -ra DEVICE_ARRAY <<< "$devices"
            device_json="["
            for i in "${!DEVICE_ARRAY[@]}"; do
                device_json+="\"${DEVICE_ARRAY[$i]}\""
                if [ $i -lt $((${#DEVICE_ARRAY[@]} - 1)) ]; then
                    device_json+=","
                fi
            done
            device_json+="]"
        fi

        echo ""
        echo "Creating account..."

        response=$(curl -s -X POST "$API_URL/api/create-account" \
            -H "Content-Type: application/json" \
            -d "{
                \"admin_password\": \"$admin_pwd\",
                \"new_username\": \"$new_user\",
                \"new_password\": \"$new_pwd\",
                \"role\": \"$role\",
                \"can_modify_settings\": $can_modify,
                \"device_ids\": $device_json
            }")

        echo ""
        echo "Response:"
        echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
        ;;

    2)
        echo "=== List All Accounts ==="
        read -p "Admin password: " -s admin_pwd
        echo ""
        echo ""
        echo "Fetching accounts..."

        response=$(curl -s -X POST "$API_URL/api/list-accounts" \
            -H "Content-Type: application/json" \
            -d "{\"admin_password\": \"$admin_pwd\"}")

        echo ""
        echo "Response:"
        echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
        ;;

    3)
        echo "=== Test Login ==="
        read -p "Username: " username
        read -p "Password: " -s password
        echo ""
        read -p "Device ID: " device_id

        echo ""
        echo "Testing login..."

        response=$(curl -s -X POST "$API_URL/api/login" \
            -H "Content-Type: application/json" \
            -d "{
                \"username\": \"$username\",
                \"password\": \"$password\",
                \"device_id\": \"$device_id\"
            }")

        echo ""
        echo "Response:"
        echo "$response" | python3 -m json.tool 2>/dev/null || echo "$response"
        ;;

    4)
        echo "=== API Server Health Check ==="
        response=$(curl -s "$API_URL/health")

        if [ "$response" == "OK" ]; then
            echo "✓ API Server is running"
            echo "✓ URL: $API_URL"
        else
            echo "✗ API Server is not responding"
            echo "  Response: $response"
        fi
        ;;

    *)
        echo "Invalid option"
        exit 1
        ;;
esac

echo ""
echo "============================================"
